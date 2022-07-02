use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64,
    auth: UnorderedMap<AccountIdHash, bool>,
}

pub type AccountIdHash = Vec<u8>;


#[near_bindgen]
impl State {
    #[init]
    pub fn new(count: u64) -> Self{
        assert!(!env::state_exists(), "Already initialized");
        Self {
            count,
            auth: UnorderedMap::new(b"Auth: ".to_vec()),
        }
    }

    pub fn check_access(&self, account_id: AccountId) -> bool{
        let account_hash = env::sha256(account_id.as_bytes());
        return match self.auth.get(&account_hash) {
            Some(status) => status,
            None => false,
        };
    }

    pub fn add_access(&mut self, account_id: AccountId) {
        let account_hash = env::sha256(account_id.as_bytes());
        self.auth.insert(&account_hash, &true);
    }

    pub fn remove_access(&mut self, account_id: AccountId){
        let account_hash = env::sha256(account_id.as_bytes());
        self.auth.remove(&account_hash);
    }

    pub fn get_num(&self) -> u64{
        return self.count;
    }

    pub fn increment(&mut self){
        self.only_auth();
        self.count += 1;
    }

    pub fn decrement(&mut self){
        self.only_auth();
        self.count -= 1;
    }

    pub fn reset(&mut self){
        self.only_auth();
        self.count = 0;
    }

    fn only_auth(&mut self) {
        if !self.check_access(env::predecessor_account_id()) {
            env::panic(b"Access does not exist.");
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn joe() -> AccountId {
        "joe.testnet".to_string()
    }
    fn robert() -> AccountId {
        "robert.testnet".to_string()
    }
    fn mike() -> AccountId {
        "mike.testnet".to_string()
    }

    // part of writing unit tests is setting up a mock context
    // this is a useful list to peek at when wondering what's available in env::*
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn increment() {
        testing_env!(get_context(joe(), 0));
        let mut contract = State::new(0);
        contract.add_access(joe());
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    #[should_panic(
        expected = r#"Access does not exist."#
    )]
    fn revert_increment() {
        testing_env!(get_context(joe(), 0));
        let mut contract = State::new(0);
        contract.add_access(mike());
        contract.increment();
        assert_eq!(1, contract.get_num());
    }


    #[test]
    fn decrement(){
        testing_env!(get_context(joe(), 0));
        let mut contract = State::new(1);
        contract.add_access(joe());
        contract.decrement();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        testing_env!(get_context(joe(), 0));
        let mut contract = State::new(0);
        contract.add_access(joe());

        contract.increment();

        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = State::new ( u64::MAX );
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow(){
        let mut contract = State::new(0);
        contract.decrement();
    }
}
