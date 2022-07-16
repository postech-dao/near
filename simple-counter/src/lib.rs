use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{near_bindgen, AccountId, BorshStorageKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    /// A counting number
    count: u64,
    /// Authorized transaction submitters
    auth: UnorderedSet<AccountId> // An unordered map (instead of a list) would be proper as well
}

struct Transaction {
    value: u64,
    from: AccountId
}
#[derive(BorshStorageKey, BorshSerialize)]
enum UnorderedSetKey {
    AccountIdStorageKey
}

#[near_bindgen]
impl State {
    pub fn new(count: u64, auth: Vec<AccountId>) -> Self {
        let mut authSet = UnorderedSet::new(UnorderedSetKey::AccountIdStorageKey);
        for element in auth {
            authSet.insert(&element);
        }
        State { 
            count: count, 
            auth: authSet 
        }
    }

    pub fn get_num(&self) -> u64 {
        self.count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1
    }

    pub fn reset(&mut self) {
        self.count = 0
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

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
    fn check_transaction_from_contains_in_auth() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(0, auth);
        // When
        // Then
    }

    #[test]
    fn check_transaction_value_is_over_10() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(0, auth);
        // When
        // Then
    }


    #[test]
    fn increment() {
        let context = get_contract_context(accounts(0));
        testing_env!(context.build());
        // Given
        let auth = vec![accounts(0)];
        let mut state: State = State::new(0, auth);
        // When
        state.increment();
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
        // When
        state.decrement();
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
        state.increment();
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
        // When
        state.decrement();
        // Then
    }
}
