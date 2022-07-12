use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    full_node_url: String,
    account_public: String,
    account_private: String,
    wasm_binary_path: String,
    // and so on
}

impl Config {
    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string(
<<<<<<< HEAD
                std::env::var("ENVIRONMENT")
                .expect("Environment variable for the config file path is missing"),
=======
                std::env::var("TEST_CONFIG")
                    .expect("Environment variable for the config file path is missing"),
>>>>>>> 148b834c355d2c7b613050cba51c84fb2758a540
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

#[tokio::test]
#[ignore]
async fn check_connection() {
    //let data: Vec<String> = std::env::args().collect();
    let _config = Config::read_from_env();
    //println!("{:?}",data);
    unimplemented!();
}

#[tokio::test]
#[ignore]
async fn check_block_number() {
    let _config = Config::read_from_env();
    // check the latest block number recognized by the full node **twice** with some delay,
    // so that we can assure that the full node is properly updating its blocks
    unimplemented!();
}

#[tokio::test]
#[ignore]
async fn check_account() {
    let _config = Config::read_from_env();
    // by requesting the full node, check whether the account given by the config has enough native token to pay gas fee
    unimplemented!();
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
