use candid::CandidType;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, TransferArgs,
};
use ic_principal::Principal;
use serde::Deserialize;

// const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const ICP_LEDGER_CANISTER_ID: &str = "bkyz2-fmaaa-aaaaa-qaaaq-cai";

fn extract_subaccount(account_id: AccountIdentifier) -> Option<Subaccount> {
    None
}

pub type Result<T> = std::result::Result<T, String>;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferRequest {
    to: String,
    amount: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BalanceRequest {
    account: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferResponse {
    block_index: Option<BlockIndex>,
    error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BalanceResponse {
    balance: u64, // Balance in e8s
    error: Option<String>,
}

pub async fn transfer(
    to_account: AccountIdentifier, // litpa's account
    amount: u64,                   // Amount in e8s (ICP has 8 decimal places)
) -> Result<TransferResponse> {
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

    match ic_cdk::api::call::call(
        Principal::from_text(ICP_LEDGER_CANISTER_ID).unwrap(),
        "send_dfx",
        (args,),
    )
    .await
    .map_err(|e| e.1)
    {
        Ok((block_index,)) => Ok(TransferResponse {
            block_index: Some(block_index),
            error: None,
        }),
        Err(e) => Ok(TransferResponse {
            block_index: None,
            error: Some(e),
        }),
    }
}

pub async fn check_balance(account: String) -> Result<BalanceResponse> {
    let account_identifier = AccountIdentifier::from_hex(&account).map_err(|e| e.to_string())?;

    let balance_args = AccountBalanceArgs {
        account: account_identifier,
    };

    match ic_cdk::api::call::call::<(AccountBalanceArgs,), (Tokens,)>(
        Principal::from_text(ICP_LEDGER_CANISTER_ID).unwrap(),
        "account_balance_dfx",
        (balance_args,),
    )
    .await
    .map_err(|e| e.1)
    {
        Ok((icpts,)) => {
            // Log the balance in e8s (the smallest unit of ICP)
            ic_cdk::println!("Received balance: {} e8s", icpts.e8s());

            Ok(BalanceResponse {
                balance: icpts.e8s(),
                error: None,
            })
        }
        Err(e) => {
            // Log the error
            ic_cdk::println!("Error fetching balance: {:?}", e);

            Ok(BalanceResponse {
                balance: 0,
                error: Some(e),
            })
        }
    }
}
