use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

#[derive(Default)]
struct RuntimeState {
    data: Data,
}

#[derive(Default)]
struct Data {
    todos: Vec<TodoItem>,
}

struct TodoItem {
    id: u32,
    done: bool,
    name: String,
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
