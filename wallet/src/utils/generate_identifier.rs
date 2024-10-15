use candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use sha2::{Digest, Sha256};

fn generate_sub_account(user_id: String, canister_id: Principal) -> Subaccount {
    let mut hash = Sha256::new();
    hash.update(canister_id.as_slice());
    hash.update(user_id.as_bytes());

    let hash_result = hash.finalize();
    let sub_account_bytes: [u8; 32] = hash_result.as_slice()[0..32]
        .try_into()
        .expect("Slice conversion failed");

    Subaccount(sub_account_bytes)
}

pub fn generate_account_identifier(user_id: String, canister_id: Principal) -> AccountIdentifier {
    let subaccount = generate_sub_account(user_id, canister_id);

    ic_cdk::println!("sub account: {:?}", subaccount);

    AccountIdentifier::new(&canister_id, &subaccount)
}
