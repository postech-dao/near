use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64,
    auth_ids: UnorderedSet<AccountId>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    from: AccountId,
    value: u64,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    AuthAccount,
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

    pub fn validate_transaction(&self, trans: &Transaction) -> bool {
        self.authenticate_id(&trans.from) && State::check_transaction_value(trans.value)
    }

    pub fn authenticate_id(&self, from_account: &AccountId) -> bool {
        if self.auth_ids.contains(from_account) {
            true
        } else {
            panic!(
                "Validation failed: account {} is not in auth_ids",
                from_account
            );
        }
    }

    pub fn check_transaction_value(value: u64) -> bool {
        let max_transaction_value = 10;
        if value <= max_transaction_value {
            true
        } else {
            panic!(
                "Validation failed: transaction value {} larger than max_transaction value {}",
                value, max_transaction_value
            )
        }
    }

    pub fn get_num(&self) -> u64 {
        self.count
    }

    pub fn increment(&mut self, trans: &Transaction) {
        if self.validate_transaction(trans) {
            self.count += trans.value;
        }
    }

    pub fn decrement(&mut self, trans: &Transaction) {
        if self.validate_transaction(trans) {
            self.count -= trans.value;
        }
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

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn increment() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction {
            from: accounts(0),
            value: 5,
        };

        contract.increment(&transaction);
        assert_eq!(15, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction {
            from: accounts(0),
            value: 5,
        };

        contract.decrement(&transaction);
        assert_eq!(5, contract.get_num());
    }

    #[test]
    fn reset() {
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn test_validate_auth() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction {
            from: accounts(1),
            value: 5,
        };
        contract.increment(&transaction);
    }

    #[test]
    #[should_panic]
    fn test_validate_max_value() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let auths = vec![accounts(0)];
        let mut contract = State::new(10, auths);
        let transaction = Transaction {
            from: accounts(0),
            value: 100,
        };
        contract.increment(&transaction);
    }
}
