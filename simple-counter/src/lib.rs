use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64,
}

#[near_bindgen]
impl State {
    #[init]
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
        self.count -= 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn increment() {
        let mut contract = State::new(0);
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = State::new(1);
        contract.decrement();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        let mut contract = State::new(5);
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = State::new(u64::MAX);
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        let mut contract = State::new(u64::MIN);
        contract.decrement();
    }
}
