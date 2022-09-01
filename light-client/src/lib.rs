use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{near_bindgen, BorshStorageKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LightClient {
    pub light_client_vector: Vector<pdao_colony_contract_common::LightClient>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    LightClient,
}

impl Default for LightClient {
    fn default() -> Self {
        Self {
            light_client_vector: Vector::new(StorageKey::LightClient),
        }
    }
}

#[near_bindgen]
impl LightClient {
    #[init]
    pub fn new() -> Self {
        Self {
            light_client_vector: Vector::new(StorageKey::LightClient),
        }
    }

    pub fn update_light_client(
        &mut self,
        message: pdao_beacon_chain_common::message::DeliverableMessage,
    ) {
        let light_client = pdao_colony_contract_common::LightClient::new(
            "initial_header".to_string(),
            "near".to_string(),
        );
        self.light_client_vector.push(&light_client);
    }

    pub fn get_light_client_header(&self) -> pdao_colony_contract_common::Header {
        if let Some(lastest_light_client) = self
            .light_client_vector
            .get(self.light_client_vector.len() - 1)
        {
            lastest_light_client.last_header
        } else {
            "initial_header".to_string()
        }
    }

    pub fn verify_commitment(
        &self,
        message: pdao_beacon_chain_common::message::DeliverableMessage,
    ) -> bool {
        let merkle_proof = "merkle_proof".to_string();
        let mut result = false;
        for i in 0..self.light_client_vector.len() {
            if let Some(light_client) = self.light_client_vector.get(i) {
                result = light_client.verify_commitment(message.clone(), i, merkle_proof.clone());
                if result {
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn transaction() {
        assert!(true);
    }
}
