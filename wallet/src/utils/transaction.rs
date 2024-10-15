use ic_cdk::api::call::call;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs,
};
use ic_principal::Principal;

const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

fn extract_subaccount(account_id: AccountIdentifier) -> Option<Subaccount> {
    None
}

pub async fn transfer(
    to_account: AccountIdentifier, // litpa's account
    amount: u64,                   // Amount in e8s (ICP has 8 decimal places)
) -> Result<(), String> {
    let token_amount = Tokens::from_e8s(amount); // Amount in token (1 ICP = 100,000,000 e8s)
    let fees = Tokens::from_e8s(10_000);

    // let from_subaccount = extract_subaccount(from_account);

    let args = TransferArgs {
        memo: Memo(amount),
        amount: token_amount,
        fee: fees,
        from_subaccount: None,
        to: to_account,
        created_at_time: None,
    };

    let result: Result<(Result<(), String>,), _> = call(
        Principal::from_text(ICP_LEDGER_CANISTER_ID).unwrap(),
        "icrc1_transfer",
        (args,),
    )
    .await
    .map_err(|e| format!("Failed to send ICP: {:?}", e));

    match result {
        Ok((Ok(()),)) => Ok(()),
        Ok((Err(err_msg),)) => Err(err_msg),
        Err(e) => Err(format!("Failed to decode the result: {}", e)),
    }
}

pub async fn check_balance(account: String) -> Result<Tokens, String> {
    let account_bytes = match hex::decode(&account) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(format!(
                "Invalid hex string for Account Identifier: {:?}",
                e
            ))
        }
    };

    let account_identifier = AccountIdentifier::from_slice(&account_bytes)
        .map_err(|e| format!("invalid Account Identifier: {}", e))?;

    let balance_args = AccountBalanceArgs {
        account: account_identifier,
    };

    let result: Result<(Tokens,), _> = call(
        Principal::from_text(ICP_LEDGER_CANISTER_ID).unwrap(),
        "account_balance_dfx",
        (balance_args,),
    )
    .await
    .map_err(|e| format!("Failed to check balance: {:?}", e));

    match result {
        Ok((balance,)) => Ok(balance),
        Err(e) => Err(format!("Failed to decode the result: {:?}", e)),
    }
}
