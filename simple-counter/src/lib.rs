use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    /// A counting number
    count: u64,
}

#[near_bindgen]
impl State {
    pub fn new(count: u64) -> Self {
        State { count }
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

    #[test]
    fn increment() {
        // Given
        let mut state: State = State::new(0);
        state.reset();
        // When
        state.increment();
        // Then
        assert_eq!(1, state.get_num());
    }

    #[test]
    fn decrement() {
        // Given
        let mut state: State = State::new(1);
        // When
        state.decrement();
        // Then
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
        let mut state: State = State::new(u64::max_value());
        // When
        state.increment();
        // Then
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        // Given
        let mut state: State = State::new(u64::min_value());
        // When
        state.decrement();
        // Then
    }
}
