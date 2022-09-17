use near_sdk::serde::{Deserialize, Serialize};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../out/simple_counter.wasm";

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    value: u64,
}

async fn init() -> anyhow::Result<(Account, Account, Transaction, Contract, Worker<Sandbox>)> {
    let sandbox = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = sandbox.dev_deploy(&wasm).await?;

    let owner = sandbox.root_account().unwrap();
    let alice = owner
        .create_subaccount(&sandbox, "alice")
        .transact()
        .await?
        .into_result()?;

    let bob = owner
        .create_subaccount(&sandbox, "bob")
        .transact()
        .await?
        .into_result()?;

    let transaction = Transaction { value: 10 };

    contract
        .call(&sandbox, "new")
        .args_json(json!({"val": 100, "auths": vec![alice.id()]}))?
        .transact()
        .await?;

    Ok((alice, bob, transaction, contract, sandbox))
}

#[tokio::test]
async fn test_increment() -> anyhow::Result<()> {
    let (alice, _, transaction, contract, sandbox) = init().await?;

    let start_counter: u64 = alice
        .call(&sandbox, contract.id(), "get_num")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    alice
        .call(&sandbox, contract.id(), "increment")
        .args_json(json!({ "trans": transaction }))?
        .transact()
        .await?;

    let end_counter: u64 = alice
        .call(&sandbox, contract.id(), "get_num")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    assert_eq!(end_counter, start_counter + transaction.value);
    println!("Increment ✅");
    Ok(())
}

#[tokio::test]
async fn test_decrement() -> anyhow::Result<()> {
    let (alice, _, transaction, contract, sandbox) = init().await?;

    let start_counter: u64 = alice
        .call(&sandbox, contract.id(), "get_num")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    alice
        .call(&sandbox, contract.id(), "decrement")
        .args_json(json!({ "trans": transaction }))?
        .transact()
        .await?;

    let end_counter: u64 = alice
        .call(&sandbox, contract.id(), "get_num")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    assert_eq!(end_counter, start_counter - transaction.value);
    println!("Decrement ✅");
    Ok(())
}

#[tokio::test]
async fn test_reset() -> anyhow::Result<()> {
    let (alice, _, _, contract, sandbox) = init().await?;

    alice
        .call(&sandbox, contract.id(), "reset")
        .args_json(json!({}))?
        .transact()
        .await?;

    let reset_counter: u64 = alice
        .call(&sandbox, contract.id(), "get_num")
        .args_json(json!({}))?
        .transact()
        .await?
        .json()?;

    assert_eq!(reset_counter, 0);
    println!("Reset ✅");
    Ok(())
}
