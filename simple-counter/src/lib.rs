use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{near_bindgen, AccountId, BorshStorageKey};
use serde::Serialize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    /// A counting number
    count: u64,
    /// Authorized transaction submitters
    auth: UnorderedSet<AccountId>, // An unordered map (instead of a list) would be proper as well
}

#[derive(Serialize)]
pub struct Transaction {
    value: u64,
    from: AccountId,
}
#[derive(BorshStorageKey, BorshSerialize)]
enum UnorderedSetKey {
    AccountIdStorageKey,
}

#[near_bindgen]
impl State {
    pub fn new(count: u64, auth: Vec<AccountId>) -> Self {
        let mut auth_set = UnorderedSet::new(UnorderedSetKey::AccountIdStorageKey);
        for element in auth {
            auth_set.insert(&element);
        }
        State {
            count,
            auth: auth_set,
        }
    }

    fn is_valid_auth(&self, account_id: &AccountId) -> bool {
        self.auth.contains(account_id)
    }

    fn is_valid_value(&self, value: u64) -> bool {
        let max_value: u64 = 10;
        value > max_value
    }

    pub fn increment(&mut self, transaction: Transaction) {
        let is_valid_auth = self.is_valid_auth(&transaction.from);
        let is_valid_value = self.is_valid_value(transaction.value);
        if is_valid_auth && is_valid_value {
            self._increment(transaction.value);
        } else {
            panic!("invalid transaction")
        }
    }

    pub fn decrement(&mut self, transaction: Transaction) {
        let is_valid_auth = self.is_valid_auth(&transaction.from);
        let is_valid_value = self.is_valid_value(transaction.value);
        if is_valid_auth && is_valid_value {
            self._decrement(transaction.value);
        } else {
            panic!("invalid transaction")
        }
    }

    fn _increment(&mut self, value: u64) {
        self.count += value;
    }

    fn _decrement(&mut self, value: u64) {
        self.count -= value;
    }

    // TODO: Only auth could reset value.
    pub fn reset(&mut self) {
        self.count = 0
    }
    pub fn get_num(&self) -> u64 {
        self.count
    }

    pub fn get_auth(&self) -> Vec<AccountId> {
        self.auth.to_vec()
    }
}

#[cfg(test)]
mod tests {

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    use super::*;

    fn get_contract_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic]
    fn check_transaction_from_contains_in_auth() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let account_idx0 = accounts(0);
        let auth = vec![account_idx0];
        let mut state = State::new(0, auth);
        let account_idx1 = accounts(1);
        let transaction = Transaction {
            value: 1,
            from: account_idx1,
        };
        // When
        state.increment(transaction);
        // Then
    }

    #[test]
    #[should_panic]
    fn check_transaction_value_is_over_10() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let account_idx0 = accounts(0);
        let auth = vec![account_idx0];
        let mut state = State::new(0, auth);
        let transaction = Transaction {
            value: 11,
            from: accounts(0),
        };
        // When
        state.increment(transaction)
        // Then
    }

    #[test]
    fn increment() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let account_idx0 = accounts(0);
        let auth = vec![account_idx0];
        let mut state: State = State::new(0, auth);
        let transaction = Transaction {
            value: 1,
            from: accounts(0),
        };
        // When
        state.increment(transaction);
        // Then
        assert_eq!(1, state.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(1, auth);
        let transaction = Transaction {
            value: 1,
            from: accounts(0),
        };
        // When
        state.decrement(transaction);
        // Then
        assert_eq!(0, state.get_num());
    }

    #[test]
    fn reset() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(1, auth);
        // When
        state.reset();
        // Then
        assert_eq!(0, state.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(u64::max_value(), auth);
        // When
        let transaction = Transaction {
            value: 1,
            from: accounts(0),
        };
        // When
        state.increment(transaction);
        // Then
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(u64::min_value(), auth);
        let transaction = Transaction {
            value: 1,
            from: accounts(0),
        };
        // When
        state.decrement(transaction);
        // Then
    }

    #[test]
    fn get_auth() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let state: State = State::new(0, auth);
        // When
        let auth = state.get_auth();
        //Then
        assert_eq!(auth, vec![accounts(0)]);
    }
}
