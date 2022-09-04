use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LightClient {
    pub light_client: pdao_colony_contract_common::LightClient,
}

impl Default for LightClient {
    fn default() -> Self {
        Self {
            light_client: pdao_colony_contract_common::LightClient {
                height: 0,
                last_header: "last_header".to_string(),
                chain_name: "near".to_string(),
            },
        }
    }
}

#[near_bindgen]
impl LightClient {
    #[init]
    pub fn new() -> Self {
        Self {
            light_client: pdao_colony_contract_common::LightClient {
                height: 0,
                last_header: "last_header".to_string(),
                chain_name: "near".to_string(),
            },
        }
    }

    pub fn update(&mut self, header: pdao_colony_contract_common::Header) {
        self.light_client.height += 1;
        self.light_client.last_header = header;
    }

    pub fn get_header(&self) -> pdao_colony_contract_common::Header {
        self.light_client.last_header.clone()
    }

    pub fn verify_commitment(
        &self,
        message: pdao_beacon_chain_common::message::DeliverableMessage,
        block_height: u64,
        proof: pdao_colony_contract_common::MerkleProof,
    ) -> bool {
        self.light_client
            .verify_commitment(message, block_height, proof)
    }
}

#[cfg(test)]
mod tests {

    use crate::LightClient;

    #[test]
    fn test_update() {
        let header = "new_last_header".to_string();
        let mut light_client_contract = LightClient::new();
        light_client_contract.update(header);
        assert_eq!(
            light_client_contract.light_client.last_header,
            "new_last_header".to_string()
        )
    }

    #[test]
    fn test_get_header() {
        let light_client_contract = LightClient::new();
        assert_eq!(
            light_client_contract.get_header(),
            "last_header".to_string()
        )
    }

    #[test]
    fn test_verify_commitment() {
        let light_client_contract = LightClient::new();
        let message =
            pdao_beacon_chain_common::message::DeliverableMessage::NonFungibleTokenTransfer(
                pdao_beacon_chain_common::message::NonFungibleTokenTransfer {
                    collection_address: "collection_address".to_string(),
                    token_index: "token_index".to_string(),
                    receiver_address: "receiver_address".to_string(),
                    contract_sequence: 0,
                },
            );
        let block_height = 0;
        let proof = "valid".to_string();
        let verify_result = light_client_contract.verify_commitment(message, block_height, proof);
        assert!(verify_result)
    }
}
