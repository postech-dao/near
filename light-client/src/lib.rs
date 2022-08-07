use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use pdao_colony_contract_common::LightClientUpdateMessage;
use std::time::{SystemTime, UNIX_EPOCH};

/// TODO: replace this with the proper type.
pub type Header = String;
/// TODO: replace this with the proper type.
pub type BlockFinalizationProof = String;
/// TODO: replace this with the proper type.
pub type MerkleProof = String;
/// TODO: replace this with the proper type.
pub type Timestamp = u128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LightClient {
    pub height: u64,
    pub last_header: Header,
    pub timestamp: Timestamp,
}

impl Default for LightClient {
    fn default() -> Self {
        Self {
            height: 0,
            last_header: "".to_string(),
            timestamp: 0,
        }
    }
}

#[near_bindgen]
impl LightClient {
    #[init]
    pub fn new(initial_header: Header) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            height: 0,
            last_header: initial_header,
            timestamp: time,
        }
    }

    pub fn update(&mut self, message: LightClientUpdateMessage) -> bool {
        if message.proof.as_str() == "valid" {
            self.height += 1;
            self.last_header = message.header;
            true
        } else {
            false
        }
    }

    pub fn verify_commitment(
        &self,
        _message: Vec<u8>,
        block_height: u64,
        proof: MerkleProof,
    ) -> bool {
        proof.as_str() == "valid" && block_height == self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::LightClient;
    use pdao_colony_contract_common::LightClientUpdateMessage;

    #[test]
    fn update() {
        let mut light_client = LightClient::new(String::from("0x1"));
        let light_client_update_messeage = LightClientUpdateMessage {
            header: String::from("0x2"),
            proof: String::from("valid"),
        };
        light_client.update(light_client_update_messeage);
        assert_eq!(light_client.last_header, String::from("0x2"));
    }

    #[test]
    fn verify_commitment() {
        let mut light_client: LightClient = LightClient::new(String::from("0x1"));
        let light_client_update_messeage = LightClientUpdateMessage {
            header: String::from("0x2"),
            proof: String::from("valid"),
        };
        light_client.update(light_client_update_messeage);

        let status = light_client.verify_commitment(vec![], 1, String::from("valid"));
        assert_eq!(status, true);
    }
}
