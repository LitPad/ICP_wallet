use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

#[derive(Default)]
struct RuntimeState {
    data: Data,
}

#[derive(CandidType, Deserialize, Default)]
struct Data {
    todos: Vec<TodoItem>,
}

#[derive(CandidType, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    done: bool,
    name: String,
}

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| {
        let data = &state.borrow().data;
        ic_cdk::storage::stable_save((data,)).unwrap();
    });
    // instead of using unwrap, handle the result for prod
}

#[post_upgrade]
fn post_upgrade() {
    match ic_cdk::storage::stable_restore::<(Data,)>() {
        Ok((data,)) => {
            let runtime_state = RuntimeState { data };
            RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
        }
        Err(e) => {
            ic_cdk::println!("Failed to restore stable memory: {:?}", e);

            RUNTIME_STATE.with(|state| *state.borrow_mut() = RuntimeState::default());
        }
    }
}

#[update]
fn add(name: String) -> u32 {
    RUNTIME_STATE.with(|state| add_impl(name, &mut state.borrow_mut()))
}

fn add_impl(name: String, runtime_state: &mut RuntimeState) -> u32 {
    let id = runtime_state.data.todos.len() as u32;

    runtime_state.data.todos.push(TodoItem {
        id,
        done: false,
        name,
    });

    id
}

#[query]
fn get() -> Vec<TodoItem> {
    RUNTIME_STATE.with(|state| get_impl(&state.borrow_mut()))
}

fn get_impl(runtime_state: &RuntimeState) -> Vec<TodoItem> {
    runtime_state.data.todos.clone()
}
