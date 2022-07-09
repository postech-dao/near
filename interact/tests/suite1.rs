use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    network_id: String,
    test_node_url: String,
    account_public: String,
    account_private: String,
    wasm_binary_path: String,
    // and so on
}

impl Config {
    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string(
                std::env::var("TEST_CONFIG")
                    .expect("Environment variable for the config file path is missing"),
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

#[tokio::test]
async fn check_connection() {
    let config = Config::read_from_env();
    // check whether the full node is responding by a simple request
    let client = Client::new();
    let payload = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "status",
            "params": []
    });

    let res = client
        .post(&config.test_node_url)
        .json(&payload)
        .send()
        .await
        .unwrap();

    let json = res.json::<Value>().await.unwrap();
    assert_eq!(json["result"]["chain_id"], config.network_id);
}

#[tokio::test]
async fn check_block_number() {
    let _config = Config::read_from_env();
    // check the latest block number recognized by the full node **twice** with some delay,
    // so that we can assure that the full node is properly updating its blocks
    let client = Client::new();
    let payload = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block",
        "params": {
          "finality": "final"
        }
    });

    let first_res = client
        .post(&_config.test_node_url)
        .json(&payload)
        .send()
        .await
        .unwrap();

    let first_json = first_res.json::<Value>().await.unwrap();

    sleep(Duration::from_secs(2)).await;

    let second_res = client
        .post(&_config.test_node_url)
        .json(&payload)
        .send()
        .await
        .unwrap();

    let second_json = second_res.json::<Value>().await.unwrap();

    assert!(
        second_json["result"]["header"]["height"].as_u64()
            > first_json["result"]["header"]["height"].as_u64()
    );
}

#[tokio::test]
async fn check_account() {
    let _config = Config::read_from_env();
    // by requesting the full node, check whether the account given by the config has enough native token to pay gas fee
    let client = Client::new();
    let payload = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": {
        "request_type": "view_account",
        "finality": "final",
        "account_id": "nearkat.testnet"
        }
    });

    let res = client
        .post(&_config.test_node_url)
        .json(&payload)
        .send()
        .await
        .unwrap();

    let json = res.json::<Value>().await.unwrap();
    assert_eq!(json["error"], Value::Null);
}

#[tokio::test]
#[ignore]
async fn query_contract_state() {
    let _config = Config::read_from_env();
    // view the contract, query its state
    unimplemented!();
}

#[tokio::test]
#[ignore]
async fn modify_contract_state() {
    let _config = Config::read_from_env();
    // view the contract, submit a transaction that modifies its state
    unimplemented!();
}
