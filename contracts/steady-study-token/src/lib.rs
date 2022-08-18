// use for NEP-141 Fungible Token
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    PromiseOrValue,
};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

use near_contract_standards::fungible_token::core_impl::FungibleToken;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_sdk::json_types::ValidAccountId;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct SteadyStudyTokenContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
}

#[near_bindgen]
impl SteadyStudyTokenContract {

    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                // ft-1.0.0
                spec: FT_METADATA_SPEC.to_string(),
                name: "Steady Study Token reward your works".to_string(),
                symbol: "STEADYST".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        assert_eq!(!env::state_exists(), true, "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

}

// `ident`識別子は関数・変数の名前用の識別子
// https://doc.rust-jp.rs/rust-by-example-ja/macros/designators.html
// near_contract_standards::impl_fungible_token_core! 
// https://docs.rs/near-contract-standards/latest/src/near_contract_standards/fungible_token/macros.rs.html#4-59
// this means that impl FungibleTokenCore trait and FungibleTokenResolver trait for SteadyStudyTokenContract
near_contract_standards::impl_fungible_token_core!(SteadyStudyTokenContract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(SteadyStudyTokenContract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for SteadyStudyTokenContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

// use $crate::fungible_token::core::FungibleTokenCore;
// impl FungibleTokenCore for SteadyStudyTokenContract {
    // #[payable]
    // fn ft_transfer(
    //     &mut self,
    //     receiver_id: AccountId,
    //     amount: U128,
    //     memo: Option<String>,
    // ) {
    //     self.$token.ft_transfer(receiver_id, amount, memo)
    // }

    // #[payable]
    // fn ft_transfer_call(
    //     &mut self,
    //     receiver_id: AccountId,
    //     amount: U128,
    //     memo: Option<String>,
    //     msg: String,
    // ) -> PromiseOrValue<U128> {
    //     self.$token.ft_transfer_call(receiver_id, amount, memo, msg)
    // }

    // fn ft_total_supply(&self) -> U128 {
    //     self.$token.ft_total_supply()
    // }

    // fn ft_balance_of(&self, account_id: AccountId) -> U128 {
    //     self.$token.ft_balance_of(account_id)
    // }
// }
// use $crate::fungible_token::resolver::FungibleTokenResolver;
// impl FungibleTokenResolver for SteadyStudyTokenContract {
    // #[private]
    // fn ft_resolve_transfer(
    //     &mut self,
    //     sender_id: AccountId,
    //     receiver_id: AccountId,
    //     amount: U128,
    // ) -> U128 {
    //     let (used_amount, burned_amount) =
    //         self.$token.internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
    //     if burned_amount > 0 {
    //         $(self.$on_tokens_burned_fn(sender_id, burned_amount);)?
    //     }
    //     used_amount.into()
    // }
// }
