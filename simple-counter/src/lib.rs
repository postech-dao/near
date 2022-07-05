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
        return State { count: count };
    }

    pub fn get_num(&self) -> u64 {
        return self.count
    }

    pub fn increment(&mut self) {
        if self.count == u64::max_value() {
            panic!("Overflow occured for count, count's current value: {}, max_value: {}", self.count, u64::max_value())
        }
        self.count +=1;
    }

    pub fn decrement(&mut self) {
        if self.count == u64::min_value() {
            panic!("Underflow occured for count, count's current value: {}, min_value: {}", self.count, u64::min_value())
        }
        self.count -=1;
    }

    pub fn reset(&mut self) {
        self.count=0;
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

    #[test]
    fn increment() {
        let mut contract = State { count: 0 };
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = State { count: 1 };
        contract.decrement();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        let mut contract = State { count: 1 };
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = State { count: u64::max_value() };
        contract.increment()
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        let mut contract = State { count: u64::min_value() };
        contract.decrement()
    }
}