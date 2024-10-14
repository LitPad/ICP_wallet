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
    wallet: Vec<Wallet>,
}

#[derive(CandidType, Deserialize, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    narration: String,
    created_at: TimestampMillis,
}

#[derive(CandidType, Deserialize, Clone)]
struct Wallet {
    user_id: String,
    balance: u64,
    transaction: Vec<Transaction>,
    created_at: TimestampMillis,
    updated_at: TimestampMillis,
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
fn create_wallet(name: String) -> Wallet {
    RUNTIME_STATE.with(|state| create_wallet_impl(name, &mut state.borrow_mut()))
}

fn create_wallet_impl(user_id: String, runtime_state: &mut RuntimeState) -> Wallet {
    // Check if the user already has a wallet
    let existing_wallets = get_wallet_impl(user_id.clone(), runtime_state);

    if !existing_wallets.is_empty() {
        // User already has a wallet
        ic_cdk::println!("Failed to create wallet: user already has a wallet");
        return existing_wallets[0].clone();
    }

    // If the user doesn't have a wallet, create a new one
    let new_wallet = Wallet {
        user_id: user_id.clone(),
        balance: 0,
        transaction: vec![],
        created_at: runtime_state.env.now(),
        updated_at: runtime_state.env.now(),
    };

    runtime_state.data.wallet.push(new_wallet.clone());

    // Return the newly created wallet
    new_wallet
}

#[query]
fn get_wallet(user_id: String) -> Vec<Wallet> {
    RUNTIME_STATE.with(|state| get_wallet_impl(user_id, &state.borrow_mut()))
}

fn get_wallet_impl(user_id: String, runtime_state: &RuntimeState) -> Vec<Wallet> {
    let wallets: Vec<Wallet> = runtime_state
        .data
        .wallet
        .iter()
        .filter(|wallet| wallet.user_id == user_id)
        .cloned()
        .collect();

    wallets
}

// #[cfg(test)]
// mod tests {
//     use env::TestEnvironment;

//     use super::*;

//     #[test]
//     fn add_then_get() {
//         let mut runtime_state = RuntimeState {
//             env: Box::new(TestEnvironment { now: 1 }),
//             data: Data::default(),
//         };

//         let name = "abcd".to_string();

//         let id = add_impl(name.clone(), &mut runtime_state);

//         ic_cdk::println!("ID: {:?}", id.clone());

//         let results = get_impl(&runtime_state);

//         assert_eq!(results.len(), 1);

//         let result = results.first().unwrap();

//         assert_eq!(result.name, name);
//         assert_eq!(result.date_added, 1);
//         assert!(!result.done);
//     }
// }
