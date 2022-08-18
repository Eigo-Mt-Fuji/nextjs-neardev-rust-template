use serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*; 
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../res/steady_study_token.wasm";

// Result<T, anyhow::Error>, or equivalently anyhow::Result<T>,https://github.com/dtolnay/anyhow
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // https://docs.rs/workspaces/latest/workspaces/fn.sandbox.html
    // pub async fn sandbox() -> Result<Worker<Sandbox>>
    // エラーハンドリングのための ?演算子 エラー委譲
    // https://doc.rust-jp.rs/book-ja/appendix-02-operators.html#%E4%BB%98%E9%8C%B2b-%E6%BC%94%E7%AE%97%E5%AD%90%E3%81%A8%E8%A8%98%E5%8F%B7
    let worker = workspaces::sandbox().await?;

    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let outcome = contract
    .call(&worker, "new_default_meta")
    .args_json(json!({ "owner_id": contract.id(), "total_supply": "10000" }))?
    .transact()
    .await?;

    // outcome contains data like logs, receipts and transaction outcomes.
    println!("new_default_meta outcome: {:#?}", outcome);

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = owner
    .create_subaccount(&worker, "alice")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;

    // begin tests  
    test_ft_total_supply(&owner, &alice, &contract, &worker).await?;
    Ok(())
}   

async fn test_ft_total_supply(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    println!("call ft_total_supply");
    let alice_status: String = user
        .call(&worker, contract.id(), "ft_total_supply")
        .args_json(json!({ "account_id": user.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(alice_status, "10000");
    println!("      Passed ✅ ft_total_supply get");
    Ok(())
}
