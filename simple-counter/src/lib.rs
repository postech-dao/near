use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64, // TODO,
}

#[near_bindgen]
impl State {
    #[init]
    pub fn new(count: u64) -> Self {
        State { count } // TODO
    }

    pub fn get_num(&self) -> u64 {
        return self.count; // TODO
    }

    pub fn increment(&mut self) {
        self.count += 1; // TODO

    }

    pub fn decrement(&mut self) {
        self.count -= 1; // TODO
    }

    pub fn reset(&mut self) {
        self.count = 0; // TODO
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

    #[test]
    fn increment() {
        let mut contract: State = State::new(0); // TODO
        assert_eq!(0, contract.get_num());
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract: State = State::new(1); // TODO
        assert_eq!(1, contract.get_num());
        contract.decrement();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        let mut contract: State = State::new(5); // TODO
        assert_eq!(5, contract.get_num());
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract: State = State::new(u64::max_value()); // TODO
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        let mut contract: State = State::new(u64::min_value()); // TODO
        contract.decrement();
    }
}