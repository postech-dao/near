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
    pub fn new(count: u64) -> Self {}

    pub fn get_num(&self) -> u64 {
        // TODO
        return self.count;
    }

    pub fn increment(&mut self) {
        // TODO
        self.count += 1;
        let log_message = format!("Increased number to {}", self.count);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn decrement(&mut self) {
        // TODO
        self.count -= 1;
        let log_message = format!("Decreased number to {}", self.count);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn reset(&mut self) {
        // TODO
        self.count = 0;
        env::log(b"Reset counter to zero");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn increment() {
        // TODO
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = State { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        // TODO
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = State { val: 1 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        // TODO
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = State { val: 100 };
        contract.reset();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        // TODO
        let mut contract = State {val:9223372036854775807};
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        // TODO
        let mut contract = State {val = -9223372036854775808};
        contract.decrement();
    }
}