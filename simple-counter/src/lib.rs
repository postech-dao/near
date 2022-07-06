use near_sdk::env;

pub struct Counter{
    val: i16,
}
impl Counter {
    pub fn get_num(&self) ->i16{
        return self.val;
    }
    pub fn increment(&mut self){
        self.val +=1;
        let log_message = format!("Increased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }
    pub fn decrement(&mut self) {
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset counter to zero");
    }
}

fn after_counter_change() {
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    //use near_sdk::MockedBlockchain;
    //use near_sdk::testing_env;
    
    /*fn get_context(input: Vec<u8>, is_view: bool) -> Config {
        Config {
            full_node_url: "full_node_url".to_string(),
            account_public: "account_public".to_string(),
            account_private: "account_private".to_string(),
            wasm_binary_path: "wasm_binary_path".to_string(),
        }
    }*/
    
    #[test]
    fn increment() {
        //let context = get_context(vec![], false);
        //testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        //let context = get_context(vec![], false);
        //testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        //let context = get_context(vec![], false);
        //testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received 0 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}
