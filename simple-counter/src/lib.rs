use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    /// A counting number
    count: u64,
}

#[near_bindgen]
impl State {
    pub fn new(count: u64) -> Self {
        State { count: count }
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
    use near_sdk::{test_utils::VMContextBuilder, VMContext};

    use crate::State;

    fn get_contract_context(input: Vec<u8>, is_view: bool) -> VMContext {
        let vmContextBuilder = VMContextBuilder::new();
        return vmContextBuilder.context;
    }

    #[test]
    fn increment() {
        /// Given
        let mut state: State = State::new(0);
        state.reset();
        /// When
        state.increment();
        /// Then
        assert_eq!(1, state.get_num());
    }

    #[test]
    fn decrement() {
        /// Given
        let mut state: State = State::new(1);
        /// When
        state.decrement();
        /// Then
        assert_eq!(0, state.get_num());
    }

    #[test]
    fn reset() {
        // Given
        let mut state: State = State::new(1);
        // When
        state.reset();
        // Then
        assert_eq!(0, state.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        // Given
        let mut state: State = State::new(u64::min_value() - 1);
        // When
        state.increment();
        // Then
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        // Given
        let mut state: State = State::new(0);
        // When
        state.decrement();
        // Then
    }
}
