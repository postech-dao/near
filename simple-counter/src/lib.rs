use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter{
    user_counters: UnorderedMap<String, i16>,
}
#[near_bindgen]
impl Counter{

    #[init]
    pub fn new()-> Self{

        assert!(env::state_read::<Self>().is_none(),"Already initialized");
        Self { 
            user_counters: UnorderedMap::new(b"kmlee".to_vec()), 
        }
    }
    pub fn get_num(&self, account: String)->i16{
        let caller_num = self.get_num_from_signer(account.clone());
        let log_message = format!("{}'s number: {}",account, caller_num);
        env::log(log_message.as_bytes());
        caller_num
    }
    fn get_num_from_signer(&self, account: String) -> i16 {
        match self.user_counters.get(&account) {
            Some(num) => {
                num
            },
            None => 0
        }
    }
    pub fn increment(&mut self) {
        // real smart contracts will want to have safety checks
        //let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        let current_val = match self.user_counters.get(&caller) {
            Some(val) => val,
            None => 0i16
        };
        let new_value = current_val + 1;
        self.user_counters.insert(&caller.clone(), &new_value);
        let log_message = format!("Incremented to {}", new_value);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn decrement(&mut self) {
        //let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        let current_val = match self.user_counters.get(&caller) {
            Some(val) => val,
            None => 0i16
        };
        let new_value = current_val - 1;
        self.user_counters.insert(&caller.clone(), &new_value);

        let log_message = format!("Decreased number to {}", new_value);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn reset(&mut self) {
        //let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        self.user_counters.insert(&caller, &0i16);
        env::log(b"Reset counter to zero");
    }
    pub fn delete(&mut self, k: String) {
        env::log(b"delete");
        self.user_counters.remove(&k);
    }

    
}
fn after_counter_change() {
    env::log(b"Make sure you don't overflow, my friend.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::json_types::ValidAccountId;


    // part of writing unit tests is setting up a mock context
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(predecessor_account_id:ValidAccountId,is_view: bool) -> VMContext{
        VMContextBuilder::new()
            .signer_account_id(predecessor_account_id.clone())
            .is_view(is_view)
            .build()
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    // unlike other frameworks, the function names don't need to be special or have "test" in it
    #[test]
    fn get_num(){
        let context = get_context(accounts(1),false);
        testing_env!(context);
        // instantiate a contract variable
        let contract = Counter::new();
        println!("Get num: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received 0 when calling get_num
        assert_eq!(0, contract.get_num("kmlee".to_string()));
    }

    #[test]
    fn increment() {
        let context = get_context(accounts(1),false);
        testing_env!(context);
        // instantiate a contract variable
        let mut contract = Counter::new();
        contract.increment();
        // we can do println! in tests, but reminder to use env::log outside of tests
        println!("Value after increment: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num("kmlee".to_string()));
    }

    #[test]
    fn decrement() {
        let context = get_context( accounts(1),false);
        testing_env!(context);
        // instantiate a contract variable
        let mut contract = Counter::new();
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num("kmlee".to_string()));
    }
    #[test]
    fn reset() {
        let context = get_context( accounts(1),false);
        testing_env!(context);
        // instantiate a contract variable
        let mut contract = Counter::new();
        contract.reset();
        println!("Value after reset: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received 0 after reset
        assert_eq!(0, contract.get_num("kmlee".to_string()));
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context( accounts(1),false);
        testing_env!(context);
        // instantiate a contract variable
        let mut contract = Counter::new();
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received 0 after reset
        assert_eq!(0, contract.get_num("kmlee".to_string()));
    }
}
