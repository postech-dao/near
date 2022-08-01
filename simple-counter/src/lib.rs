use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64,
    auth_ids: UnorderedSet<AccountId>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    value: u64,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    AuthAccount,
}

impl Default for State {
    fn default() -> Self {
        Self {
            count: 0,
            auth_ids: UnorderedSet::new(StorageKey::AuthAccount),
        }
    }
}
#[near_bindgen]
impl State {
    #[init]
    pub fn new(val: u64, auths: Vec<AccountId>) -> Self {
        let mut this = Self {
            count: val,
            auth_ids: UnorderedSet::new(StorageKey::AuthAccount),
        };
        for auth_id in auths {
            this.auth_ids.insert(&auth_id);
        }

        this
    }

    pub fn validate_transaction(&self, trans: &Transaction) {
        if !self.is_valid_auth_id() {
            panic!(
                "Validation failed: account {} not in auth_ids",
                &env::predecessor_account_id()
            );
        } else if !State::is_valid_transaction_value(trans.value) {
            panic!(
                "Validation failed: transaction value {} is larger than max_transaction value",
                trans.value
            );
        }
    }

    pub fn is_valid_auth_id(&self) -> bool {
        self.auth_ids.contains(&env::predecessor_account_id())
    }

    pub fn is_valid_transaction_value(value: u64) -> bool {
        let max_transaction_value = 10;
        value <= max_transaction_value
    }

    pub fn get_num(&self) -> u64 {
        self.count
    }

    pub fn increment(&mut self, trans: &Transaction) {
        self.validate_transaction(trans);
        self.count += trans.value;
    }

    pub fn decrement(&mut self, trans: &Transaction) {
        self.validate_transaction(trans);
        self.count -= trans.value;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transaction;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn set_context(predecessor_account_id: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor_account_id);
        testing_env!(builder.build());
    }

    #[test]
    fn increment() {
        set_context(accounts(0));
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction { value: 5 };

        contract.increment(&transaction);
        assert_eq!(15, contract.get_num());
    }

    #[test]
    fn decrement() {
        set_context(accounts(0));
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction { value: 5 };

        contract.decrement(&transaction);
        assert_eq!(5, contract.get_num());
    }

    #[test]
    fn reset() {
        set_context(accounts(0));
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic(expected = "Validation failed: account bob not in auth_ids")]
    fn test_validate_auth() {
        set_context(accounts(1));
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction { value: 5 };
        contract.increment(&transaction);
    }

    #[test]
    #[should_panic(
        expected = "Validation failed: transaction value 100 is larger than max_transaction value"
    )]
    fn test_validate_max_value() {
        set_context(accounts(0));
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction { value: 100 };
        contract.increment(&transaction);
    }
}
