mod env;
mod types;
mod utils;
use candid::CandidType;
use env::{CanisterEnvironment, EmptyEnvironment, Environment};
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, Tokens};
use ic_principal::Principal;
use serde::Deserialize;
use std::cell::RefCell;
use types::TimestampMillis;
use utils::{check_balance, generate_account_identifier};

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

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    narration: String,
    created_at: TimestampMillis,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Wallet {
    user_id: String,
    balance: u64,
    transaction: Vec<Transaction>,
    address: String,
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
fn create_wallet(user_id: String) -> Wallet {
    RUNTIME_STATE.with(|state| create_wallet_impl(user_id, &mut state.borrow_mut()))
}

fn create_wallet_impl(user_id: String, runtime_state: &mut RuntimeState) -> Wallet {
    // Check if the user already has a wallet
    let existing_wallets: Vec<Wallet> = get_wallet_impl(user_id.clone(), runtime_state);

    let canister_id = Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").unwrap();

    if !existing_wallets.is_empty() {
        // User already has a wallet
        ic_cdk::println!("Failed to create wallet: user already has a wallet.");
        return existing_wallets[0].clone();
    };

    let subaccount = generate_account_identifier(user_id.clone(), canister_id);

    ic_cdk::println!("sub account address: {}", subaccount);

    let subaccount_address = subaccount.to_string();

    // let subaccount  = utils
    // If the user doesn't have a wallet, create a new one
    let new_wallet = Wallet {
        user_id: user_id.clone(),
        balance: 1,
        transaction: vec![],
        address: subaccount_address,
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

#[query]
fn get_balance(user_id: String) -> u64 {
    RUNTIME_STATE.with(|state| get_balance_impl(user_id, &state.borrow_mut()))
}

fn get_balance_impl(user_id: String, runtime_state: &RuntimeState) -> u64 {
    let wallet: Vec<Wallet> = get_wallet_impl(user_id.clone(), runtime_state);

    if wallet.is_empty() {
        ic_cdk::println!(
            "Cannot fetch wallet balance because wallet does not exist for this user!"
        );
        return 0;
    }

    let balance = wallet[0].clone().balance;
    balance
}

#[update]
fn fund_wallet(user_id: String, amount: u64) -> (u64, Transaction) {
    RUNTIME_STATE.with(|state| fund_wallet_impl(user_id, amount, &mut state.borrow_mut()))
}

fn fund_wallet_impl(
    user_id: String,
    amount: u64,
    runtime_state: &mut RuntimeState,
) -> (u64, Transaction) {
    let funded_wallet = runtime_state
        .data
        .wallet
        .iter_mut()
        .find(|wallet| wallet.user_id == user_id);

    if let Some(funded_wallet) = funded_wallet {
        funded_wallet.balance += amount;

        let transaction = Transaction {
            from: "funding".to_string(),
            to: funded_wallet.user_id.clone(),
            amount,
            narration: "Wallet has been funded".to_string(),
            created_at: runtime_state.env.now(),
        };

        funded_wallet.updated_at = runtime_state.env.now();

        funded_wallet.transaction.push(transaction.clone());

        return (funded_wallet.balance.clone(), transaction.clone());
    } else {
        ic_cdk::println!("Wallet not found");
        return (
            0,
            Transaction {
                amount: 0,
                created_at: 0,
                from: "".to_string(),
                narration: "".to_string(),
                to: "".to_string(),
            },
        );
    }
}

// e8fc6af5a6b9be901ab5fea3f6936ee60c3f30128a04c1ff6c7de584b9992b65

#[update]
async fn check_icp_balance(account: String) -> Result<Tokens, String> {
    let result = RUNTIME_STATE.with(|_state| check_balance(account)).await?;

    ic_cdk::println!("BALANCE: {:?}", result);

    result.e8s();
    return Ok(result);
}

// account id - 13f313beb13d449568ac98eb989f74b61463f7c4edb69be1b8b5d1e1044fe71a

// efault account id - 04208a95eb03b4d668859e0fc62c98cf059c0db0c1cffbe62ed5c0f3e942ff6a
#[cfg(test)]
mod tests {
    use env::TestEnvironment;

    use super::*;

    #[test]
    fn create_then_get() {
        let mut runtime_state = RuntimeState {
            env: Box::new(TestEnvironment { now: 1 }),
            data: Data::default(),
        };

        let user_id = "abcd".to_string();

        let wallet = create_wallet_impl(user_id.clone(), &mut runtime_state);

        assert_eq!(wallet.balance, 1);
        assert_eq!(wallet.user_id, user_id.clone());

        ic_cdk::println!("Wallet: {:?}", wallet.clone());

        let results = get_wallet_impl(user_id.clone(), &mut runtime_state);

        assert_eq!(results.len(), 1);

        let result = results.first().unwrap();

        assert_eq!(result.user_id, user_id.clone());
    }
}
