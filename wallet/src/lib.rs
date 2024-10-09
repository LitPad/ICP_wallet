mod env;
mod types;
use candid::CandidType;
use env::{CanisterEnvironment, EmptyEnvironment, Environment};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;
use types::TimestampMillis;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    env: Box<dyn Environment>,
    data: Data,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnvironment {}),
            data: Data::default(),
        }
    }
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
    date_added: TimestampMillis,
}

#[init]
fn init() {
    let env = Box::new(CanisterEnvironment::new());
    let data = Data::default();
    let runtime_state = RuntimeState { env, data };
    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
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
            let env = Box::new(CanisterEnvironment::new());
            let runtime_state = RuntimeState { env, data };
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
        name,
        done: false,
        date_added: runtime_state.env.now(),
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

#[cfg(test)]
mod tests {
    use env::TestEnvironment;

    use super::*;

    #[test]
    fn add_then_get() {
        let mut runtime_state = RuntimeState {
            env: Box::new(TestEnvironment { now: 1 }),
            data: Data::default(),
        };

        let name = "abcd".to_string();

        let id = add_impl(name.clone(), &mut runtime_state);

        ic_cdk::println!("ID: {:?}", id.clone());

        let results = get_impl(&runtime_state);

        assert_eq!(results.len(), 1);

        let result = results.first().unwrap();

        assert_eq!(result.name, name);
        assert_eq!(result.date_added, 1);
        assert!(!result.done);
    }
}
