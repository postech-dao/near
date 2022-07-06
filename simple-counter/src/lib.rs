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
        //TODO
        State { count }
    }

    pub fn get_num(&self) -> u64 {
        //TODO
        self.count
    }

    pub fn increment(&mut self) {
        //TODO
        self.count +=1;
    }

    pub fn decrement(&mut self) {
        //TODO
        self.count -=1;
    }

    pub fn reset(&mut self) {
        //TODO
        self.count =0;
    }
}

#[cfg(test)]
mod tests {
    use std::u64::MIN;
    use std::u64::MAX;
    use super::State;

    #[test]
    fn increment() {
        // TODO
        let mut contract = State::new(0);
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        // TODO
        let mut contract = State::new(1);
        contract.decrement();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        // TODO
        let mut contract = State::new(5);
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        // TODO
        let mut contract = State::new(MAX);
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        // TODO
        let mut contract = State::new(MIN);
        contract.decrement();
    }
}


