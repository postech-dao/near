use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::fungible_token::{events, FungibleToken};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PophinContract {
    token: FungibleToken,
    decimals: u8,
    name: String,
    symbol: String,
    icon: Option<String>,
}

near_contract_standards::impl_fungible_token_core!(PophinContract, token);
near_contract_standards::impl_fungible_token_storage!(PophinContract, token);

/// TODO: replace PDAO's pophin icon
const PDAO_POPHIN_ICON: &str = "PDAO pophin ICON ADRESSS";

#[near_bindgen]
impl PophinContract {
    /// Initializes the contract with PDAO pophin spec
    #[init]
    pub fn new() -> Self {
        Self {
            token: FungibleToken::new(b"w".to_vec()),
            decimals: 24,
            name: "PDAO_meme_token_Pophin".to_string(),
            symbol: "PDAO-POPHIN".to_string(),
            icon: Some(PDAO_POPHIN_ICON.to_string()),
        }
    }

    /// return total supply
    pub fn query_total_supply(&mut self) -> U128 {
        self.ft_total_supply()
    }

    /// return onwner's balance
    pub fn query_balance(&mut self) -> U128 {
        let owner_id = env::predecessor_account_id();
        self.token.ft_balance_of(owner_id)
    }

    pub fn mint_pophin(&mut self, receiver_id: AccountId, amount: U128) {
        // before minting validate with lightclient
        self.validate_with_lightclient();

        if self.token.accounts.get(&receiver_id).is_none() {
            self.token.internal_register_account(&receiver_id);
            self.token.internal_deposit(&receiver_id, amount.into());
        } else {
            self.token.internal_deposit(&receiver_id, amount.into());
        };

        // log mint
        events::FtMint {
            owner_id: &receiver_id,
            amount: &amount,
            memo: Some("PDAO Pophin token newly minted"),
        }
        .emit()
    }

    pub fn transfer_pophin(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        // check enough amout to transfer
        let owner_id = env::predecessor_account_id();
        let owner_balance = self.token.ft_balance_of(owner_id);
        if owner_balance < amount {
            env::panic_str("Not enough balance to transfer");
        }

        // use FungibleTokenCore::ft_transfer for internal transfer
        self.token.ft_transfer(receiver_id, amount, memo);
    }

    #[payable]
    pub fn transfer_pophin_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // check enough amout to transfer
        let owner_id = env::predecessor_account_id();
        let owner_balance = self.token.ft_balance_of(owner_id);
        if owner_balance < amount {
            env::panic_str("Not enough balance to transfer");
        }

        // use FungibleTokenCore::ft_transfer for internal transfer
        self.validate_with_lightclient();
        self.token.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    pub fn burn(&mut self, account_id: AccountId, amount: U128) {
        // validate with lightclient
        self.validate_with_lightclient();

        // check enough amount to burn
        let owner_balance = self.token.ft_balance_of(account_id.clone());
        if owner_balance < amount {
            env::panic_str("Not enough balance to burn");
        }

        self.token.internal_withdraw(&account_id, amount.into());

        // log burn
        events::FtBurn {
            owner_id: &account_id,
            amount: &amount,
            memo: Some("PDAO Pophin token burned"),
        }
        .emit()
    }
}

impl PophinContract {
    fn validate_with_lightclient(&self) {
        // todo
    }
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for PophinContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            reference: None,
            reference_hash: None,
            decimals: self.decimals,
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            icon: self.icon.clone(),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{env, testing_env};

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = PophinContract::default();
    }

    #[test]
    fn get_total_supply() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        assert_eq!(contract.query_total_supply(), 1_000_000.into());
    }

    #[test]
    fn get_balance() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        assert_eq!(contract.query_balance(), 1_000_000.into());
    }

    #[test]
    fn mint_token() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        assert_eq!(contract.ft_balance_of(accounts(0)), 1_000_000.into());
    }

    #[test]
    fn transfer_token() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());

        contract.mint_pophin(accounts(2), 1_000_000.into());

        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = 1_000_000 / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(
            contract.ft_balance_of(accounts(2)).0,
            (1_000_000 - transfer_amount)
        );
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }

    #[test]
    fn burn_token() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        assert_eq!(contract.query_total_supply(), 1_000_000.into());

        contract.burn(accounts(0), 500_000.into());
        assert_eq!(contract.ft_balance_of(accounts(0)), 500_000.into());

        assert_eq!(contract.query_total_supply(), 500_000.into());
    }

    /// test for serveral minting to a specific account.
    #[test]
    fn mint_twice() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = PophinContract::new();
        testing_env!(context
            .attached_deposit(125 * env::storage_byte_cost())
            .build());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        contract.mint_pophin(accounts(0), 1_000_000.into());
        assert_eq!(contract.ft_balance_of(accounts(0)), 2_000_000.into());
    }
}
