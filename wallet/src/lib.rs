use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    data: Data,
}

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
    RUNTIME_STATE.with(|state| state.borrow())
}
