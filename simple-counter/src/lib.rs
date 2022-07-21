use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter{
    user_counters: UnorderedMap<String, u64>,
}
#[near_bindgen]
impl Counter{

    //initiate
    #[init]
    pub fn new()-> Self{

        assert!(env::state_read::<Self>().is_none(),"Already initialized"); //check arleady existed
        Self { 
            user_counters: UnorderedMap::new(b"kmlee".to_vec()),  
        }
    }

    pub fn get_num(&self, account: String)->u64{
        let caller_num = self.get_num_from_signer(account.clone()); //return value signer have

        //logging
        let log_message = format!("{}'s number: {}",account, caller_num);
        env::log(log_message.as_bytes());
        caller_num
    }
    fn get_num_from_signer(&self, account: String) -> u64 {
        match self.user_counters.get(&account) {
            Some(num) => {
                num
            },//if the user exist return num else 0
            None => panic!("No accounts")
        }
    }
    pub fn increment(&mut self) {
        //let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        let current_val = match self.user_counters.get(&caller) { //get previous value
            Some(val) => val,
            None => panic!("No accounts")
        };
        let new_value = current_val + 1; //increasing value
        self.user_counters.insert(&caller.clone(), &new_value);

        //logging
        let log_message = format!("Incremented to {}", new_value);
        env::log(log_message.as_bytes());
    }

    pub fn decrement(&mut self) {
        let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        let current_val = match self.user_counters.get(&caller) { //get previous value
            Some(val) => val,
            None =>panic!("No accounts")
        };
        let new_value = current_val - 1; //decreasing value
        self.user_counters.insert(&caller.clone(), &new_value);

        //logging
        let log_message = format!("Decreased number to {}", new_value);
        env::log(log_message.as_bytes());
    }

    pub fn reset(&mut self) {
        //let caller = env::signer_account_id();
        //use below line instead of below for test
        let caller = "kmlee".to_string();
        self.user_counters.insert(&caller, &0u64);

        //logging
        env::log(b"Reset counter to zero");
    }
    pub fn delete(&mut self, k: String) {
        env::log(b"delete");
        self.user_counters.remove(&k);
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::json_types::ValidAccountId;


    fn get_context(predecessor_account_id:ValidAccountId,is_view: bool) -> VMContext{
        VMContextBuilder::new()
            .signer_account_id(predecessor_account_id.clone())
            .is_view(is_view)
            .build()
    }

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
        contract.increment();
        contract.increment();
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num("kmlee".to_string()));
        // confirm that we received -1 when calling get_num
        assert_eq!(1, contract.get_num("kmlee".to_string()));
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
