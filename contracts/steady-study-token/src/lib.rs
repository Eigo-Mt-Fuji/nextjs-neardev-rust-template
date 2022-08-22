#![allow(dead_code)]

// use for NEP-141 Fungible Token
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    PromiseOrValue,
};

use near_sdk::collections::UnorderedMap;
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

use near_contract_standards::fungible_token::core_impl::FungibleToken;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct SteadyStudyTokenContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    used_urls: UnorderedMap<String, AccountId>,
}

const DATA_IMAGE_SVG_ICON: &str = "data:image/svg+xml,%3C%3Fxml%20version%3D%221.0%22%20encoding%3D%22UTF-8%22%3F%3E%0A%3C!--%20Do%20not%20edit%20this%20file%20with%20editors%20other%20than%20diagrams.net%20--%3E%0A%3C!DOCTYPE%20svg%20PUBLIC%20%22-%2F%2FW3C%2F%2FDTD%20SVG%201.1%2F%2FEN%22%20%22http%3A%2F%2Fwww.w3.org%2FGraphics%2FSVG%2F1.1%2FDTD%2Fsvg11.dtd%22%3E%0A%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20version%3D%221.1%22%20width%3D%22132px%22%20height%3D%22147px%22%20viewBox%3D%22-0.5%20-0.5%20132%20147%22%20content%3D%22%26lt%3Bmxfile%20host%3D%26quot%3Bapp.diagrams.net%26quot%3B%20modified%3D%26quot%3B2022-08-22T08%3A56%3A22.102Z%26quot%3B%20agent%3D%26quot%3B5.0%20(Macintosh%3B%20Intel%20Mac%20OS%20X%2010_15_5)%20AppleWebKit%2F537.36%20(KHTML%2C%20like%20Gecko)%20Chrome%2F104.0.0.0%20Safari%2F537.36%26quot%3B%20version%3D%26quot%3B20.2.5%26quot%3B%20etag%3D%26quot%3BAojBWdFddc2SpyCW6VQz%26quot%3B%20type%3D%26quot%3Bgoogle%26quot%3B%26gt%3B%26lt%3Bdiagram%20id%3D%26quot%3Bx0MyN3F1B9d34F_WQAFN%26quot%3B%26gt%3B5ZfLjtsgFIafpQtvK2x8SZaTZGa6mGoqZdE1NsRGwcYix83l6QsxTsyQaUfKpFKVlfHP4fJ%2F5whEgOf17lmRtvouKRNBhOguwIsgisIIY%2F0xyr5XMhT2Qqk47SV0Fpb8wOzIQe04ZRur9RJIKYC3rljIpmEFOBpRSm7dsJUU1BFaUjJPWBZE%2BOpPTqGyaojQueMb42Vll54ktqMmQ7AVNhWhcjuS8GOA50pK6Fv1bs6EgedyeXqn97QxxRr4yICoH%2FCLiM56s%2FuC%2FWC2VLJrbRhTwHaXEJN8CEf%2BFsKTMV0RTNYM1F6H2ImyiV3TFkNiZ96OyGI7azWCGsaJzajNZnma%2BmxYN6zny%2F7x3%2F3rFLWmWXS5%2Fsy2FQe2bElhtK2ub61VUOsFFqFu5rJrKKMv%2BUkgxdogbOhrB4I3zOqUqPWrnoaD8Y2%2BosQVo6NqIt9lP2Yc%2FZFx6hCexj5hlPqEh0xcAzi%2BD8CDT0s4zf4d4eQ%2BCGcOYHzhkLgV4PTjgLtaPBQglbZszHJ9ZbyQnIkfcsOBy0aH5BJA1qOAB8FL0wHyTR5kD3t%2BusTQJx0GyEEZ%2BSQzHySOrweZeSD13tGC7D2e2hy4MIilVGiTTF3AV3NKzfCZYht%2BsPeRIdZK3sBx18ksSBZmrg50Pg5DGW9AybXGLEzeFo009T1bcSHeSJ%2BAPnaPiTCcevDjqQ9%2FKMFr4E88%2BPi%2B4Sf%2BLXgr9lOP%2FRIYofsv%2Fwf7lWxgafcYXpOIEeo0vA3q4YkwYh3dWZ2nbp1PbnbG6N%2FzS%2BXYN3rv4cff%26lt%3B%2Fdiagram%26gt%3B%26lt%3B%2Fmxfile%26gt%3B%22%20style%3D%22background-color%3A%20rgb(255%2C%20255%2C%20255)%3B%22%3E%3Cdefs%2F%3E%3Cg%3E%3Cpath%20d%3D%22M%206%2094%20L%2092%2094%20L%20112%20114%20L%20112%20145%20L%2026%20145%20L%206%20125%20L%206%2094%20Z%22%20fill%3D%22rgb(255%2C%20255%2C%20255)%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%206%2094%20L%2092%2094%20L%20112%20114%20L%2026%20114%20Z%22%20fill-opacity%3D%220.05%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%206%2094%20L%2026%20114%20L%2026%20145%20L%206%20125%20Z%22%20fill-opacity%3D%220.1%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2026%20145%20L%2026%20114%20L%206%2094%20M%2026%20114%20L%20112%20114%22%20fill%3D%22none%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2024%2067%20L%20110%2067%20L%20130%2087%20L%20130%20118%20L%2044%20118%20L%2024%2098%20L%2024%2067%20Z%22%20fill%3D%22rgb(255%2C%20255%2C%20255)%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2024%2067%20L%20110%2067%20L%20130%2087%20L%2044%2087%20Z%22%20fill-opacity%3D%220.05%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2024%2067%20L%2044%2087%20L%2044%20118%20L%2024%2098%20Z%22%20fill-opacity%3D%220.1%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2044%20118%20L%2044%2087%20L%2024%2067%20M%2044%2087%20L%20130%2087%22%20fill%3D%22none%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%207%2031%20L%2093%2031%20L%20113%2051%20L%20113%2082%20L%2027%2082%20L%207%2062%20L%207%2031%20Z%22%20fill%3D%22rgb(255%2C%20255%2C%20255)%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%207%2031%20L%2093%2031%20L%20113%2051%20L%2027%2051%20Z%22%20fill-opacity%3D%220.05%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%207%2031%20L%2027%2051%20L%2027%2082%20L%207%2062%20Z%22%20fill-opacity%3D%220.1%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2027%2082%20L%2027%2051%20L%207%2031%20M%2027%2051%20L%20113%2051%22%20fill%3D%22none%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cellipse%20cx%3D%2268.5%22%20cy%3D%226.25%22%20rx%3D%224.249999999999999%22%20ry%3D%224.249999999999999%22%20fill%3D%22rgb(255%2C%20255%2C%20255)%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20pointer-events%3D%22none%22%2F%3E%3Cpath%20d%3D%22M%2068.5%2010.5%20L%2068.5%2024.67%20M%2068.5%2013.33%20L%2060%2013.33%20M%2068.5%2013.33%20L%2077%2013.33%20M%2068.5%2024.67%20L%2060%2036%20M%2068.5%2024.67%20L%2077%2036%22%20fill%3D%22none%22%20stroke%3D%22rgb(0%2C%200%2C%200)%22%20stroke-miterlimit%3D%2210%22%20pointer-events%3D%22none%22%2F%3E%3Cg%20transform%3D%22translate(-0.5%20-0.5)scale(0.9999999999999999)%22%3E%3Cswitch%3E%3CforeignObject%20pointer-events%3D%22none%22%20width%3D%22101%25%22%20height%3D%22101%25%22%20requiredFeatures%3D%22http%3A%2F%2Fwww.w3.org%2FTR%2FSVG11%2Ffeature%23Extensibility%22%20style%3D%22overflow%3A%20visible%3B%20text-align%3A%20left%3B%22%3E%3Cdiv%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxhtml%22%20style%3D%22display%3A%20flex%3B%20align-items%3A%20unsafe%20center%3B%20justify-content%3A%20unsafe%20center%3B%20width%3A%201px%3B%20height%3A%201px%3B%20padding-top%3A%20132px%3B%20margin-left%3A%2069px%3B%22%3E%3Cdiv%20data-drawio-colors%3D%22color%3A%20rgb(0%2C%200%2C%200)%3B%20%22%20style%3D%22box-sizing%3A%20border-box%3B%20font-size%3A%200px%3B%20text-align%3A%20center%3B%22%3E%3Cdiv%20style%3D%22display%3A%20inline-block%3B%20font-size%3A%2012px%3B%20font-family%3A%20Helvetica%3B%20color%3A%20rgb(0%2C%200%2C%200)%3B%20line-height%3A%201.2%3B%20pointer-events%3A%20none%3B%20white-space%3A%20nowrap%3B%22%3E1%20Day%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2FforeignObject%3E%3Ctext%20x%3D%2269%22%20y%3D%22136%22%20fill%3D%22rgb(0%2C%200%2C%200)%22%20font-family%3D%22Helvetica%22%20font-size%3D%2212px%22%20text-anchor%3D%22middle%22%3E1%20Day%3C%2Ftext%3E%3C%2Fswitch%3E%3C%2Fg%3E%3Cg%20transform%3D%22translate(-0.5%20-0.5)scale(0.9999999999999999)%22%3E%3Cswitch%3E%3CforeignObject%20pointer-events%3D%22none%22%20width%3D%22101%25%22%20height%3D%22101%25%22%20requiredFeatures%3D%22http%3A%2F%2Fwww.w3.org%2FTR%2FSVG11%2Ffeature%23Extensibility%22%20style%3D%22overflow%3A%20visible%3B%20text-align%3A%20left%3B%22%3E%3Cdiv%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxhtml%22%20style%3D%22display%3A%20flex%3B%20align-items%3A%20unsafe%20center%3B%20justify-content%3A%20unsafe%20center%3B%20width%3A%201px%3B%20height%3A%201px%3B%20padding-top%3A%2067px%3B%20margin-left%3A%2069px%3B%22%3E%3Cdiv%20data-drawio-colors%3D%22color%3A%20rgb(0%2C%200%2C%200)%3B%20%22%20style%3D%22box-sizing%3A%20border-box%3B%20font-size%3A%200px%3B%20text-align%3A%20center%3B%22%3E%3Cdiv%20style%3D%22display%3A%20inline-block%3B%20font-size%3A%2012px%3B%20font-family%3A%20Helvetica%3B%20color%3A%20rgb(0%2C%200%2C%200)%3B%20line-height%3A%201.2%3B%20pointer-events%3A%20none%3B%20white-space%3A%20nowrap%3B%22%3E3%20Day%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2FforeignObject%3E%3Ctext%20x%3D%2269%22%20y%3D%2271%22%20fill%3D%22rgb(0%2C%200%2C%200)%22%20font-family%3D%22Helvetica%22%20font-size%3D%2212px%22%20text-anchor%3D%22middle%22%3E3%20Day%3C%2Ftext%3E%3C%2Fswitch%3E%3C%2Fg%3E%3Cg%20transform%3D%22translate(-0.5%20-0.5)scale(0.9999999999999999)%22%3E%3Cswitch%3E%3CforeignObject%20pointer-events%3D%22none%22%20width%3D%22101%25%22%20height%3D%22101%25%22%20requiredFeatures%3D%22http%3A%2F%2Fwww.w3.org%2FTR%2FSVG11%2Ffeature%23Extensibility%22%20style%3D%22overflow%3A%20visible%3B%20text-align%3A%20left%3B%22%3E%3Cdiv%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxhtml%22%20style%3D%22display%3A%20flex%3B%20align-items%3A%20unsafe%20center%3B%20justify-content%3A%20unsafe%20center%3B%20width%3A%201px%3B%20height%3A%201px%3B%20padding-top%3A%2013px%3B%20margin-left%3A%2031px%3B%22%3E%3Cdiv%20data-drawio-colors%3D%22color%3A%20rgb(0%2C%200%2C%200)%3B%20%22%20style%3D%22box-sizing%3A%20border-box%3B%20font-size%3A%200px%3B%20text-align%3A%20center%3B%22%3E%3Cdiv%20style%3D%22display%3A%20inline-block%3B%20font-size%3A%2012px%3B%20font-family%3A%20Helvetica%3B%20color%3A%20rgb(0%2C%200%2C%200)%3B%20line-height%3A%201.2%3B%20pointer-events%3A%20none%3B%20font-weight%3A%20bold%3B%20white-space%3A%20nowrap%3B%22%3ESteady!%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2FforeignObject%3E%3Ctext%20x%3D%2231%22%20y%3D%2217%22%20fill%3D%22rgb(0%2C%200%2C%200)%22%20font-family%3D%22Helvetica%22%20font-size%3D%2212px%22%20text-anchor%3D%22middle%22%20font-weight%3D%22bold%22%3ESteady!%3C%2Ftext%3E%3C%2Fswitch%3E%3C%2Fg%3E%3Cg%20transform%3D%22translate(-0.5%20-0.5)scale(0.9999999999999999)%22%3E%3Cswitch%3E%3CforeignObject%20pointer-events%3D%22none%22%20width%3D%22101%25%22%20height%3D%22101%25%22%20requiredFeatures%3D%22http%3A%2F%2Fwww.w3.org%2FTR%2FSVG11%2Ffeature%23Extensibility%22%20style%3D%22overflow%3A%20visible%3B%20text-align%3A%20left%3B%22%3E%3Cdiv%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxhtml%22%20style%3D%22display%3A%20flex%3B%20align-items%3A%20unsafe%20center%3B%20justify-content%3A%20unsafe%20center%3B%20width%3A%201px%3B%20height%3A%201px%3B%20padding-top%3A%20102px%3B%20margin-left%3A%2089px%3B%22%3E%3Cdiv%20data-drawio-colors%3D%22color%3A%20rgb(0%2C%200%2C%200)%3B%20%22%20style%3D%22box-sizing%3A%20border-box%3B%20font-size%3A%200px%3B%20text-align%3A%20center%3B%22%3E%3Cdiv%20style%3D%22display%3A%20inline-block%3B%20font-size%3A%2012px%3B%20font-family%3A%20Helvetica%3B%20color%3A%20rgb(0%2C%200%2C%200)%3B%20line-height%3A%201.2%3B%20pointer-events%3A%20none%3B%20white-space%3A%20nowrap%3B%22%3E2%20Day%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2Fdiv%3E%3C%2FforeignObject%3E%3Ctext%20x%3D%2289%22%20y%3D%22106%22%20fill%3D%22rgb(0%2C%200%2C%200)%22%20font-family%3D%22Helvetica%22%20font-size%3D%2212px%22%20text-anchor%3D%22middle%22%3E2%20Day%3C%2Ftext%3E%3C%2Fswitch%3E%3C%2Fg%3E%3C%2Fg%3E%3Cswitch%3E%3Cg%20requiredFeatures%3D%22http%3A%2F%2Fwww.w3.org%2FTR%2FSVG11%2Ffeature%23Extensibility%22%2F%3E%3Ca%20transform%3D%22translate(0%2C-5)%22%20xlink%3Ahref%3D%22https%3A%2F%2Fwww.diagrams.net%2Fdoc%2Ffaq%2Fsvg-export-text-problems%22%20target%3D%22_blank%22%3E%3Ctext%20text-anchor%3D%22middle%22%20font-size%3D%2210px%22%20x%3D%2250%25%22%20y%3D%22100%25%22%3EText%20is%20not%20SVG%20-%20cannot%20display%3C%2Ftext%3E%3C%2Fa%3E%3C%2Fswitch%3E%3C%2Fsvg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
    UsedUrls,
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
                name: "[Steady Study Token]reward your works".to_string(),
                symbol: "STEADYST".to_string(),
                icon: Some(DATA_IMAGE_SVG_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            }
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            // near_contract_standards::fungible_token::core_impl::FungibleToken's new 
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            used_urls: UnorderedMap::new(StorageKey::UsedUrls),
        };
        
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }
    
    ///
    /// ft_report_study_commit 
    /// 
    pub fn ft_report_study_commit(&mut self, receiver_id: AccountId, memo: Option<String>, urls: &Vec<String>) {
        
        // https://docs.rs/near-sdk/2.0.1/near_sdk/collections/struct.Vector.html#method.len
        require!(urls.len() == 3, "Need 3 urls commit.");

        // msg内のurlsがの消込済みかを判定する
        // https://docs.rs/near-sdk/2.0.1/near_sdk/collections/struct.Vector.html#method.iter
        for element in urls.iter() {
            require!(self.used_urls.get(element) == None, "Not allows same url commit.");
        }
        let sender_account_id = env::signer_account_id();

        // URL消込
        for element in urls.iter() {
            self.used_urls.insert(element, &sender_account_id);
        }

        // 
        let owner_account_id = env::current_account_id();
        println!("owner_account_id: {}", &owner_account_id);
        if self.token.accounts.get(&receiver_id) == None {
            self.token.internal_register_account(&receiver_id);
        }
        self.token.internal_transfer(
            &owner_account_id,
            &receiver_id,
            1,
            memo
        );    
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }
    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}


#[near_bindgen]
impl FungibleTokenMetadataProvider for SteadyStudyTokenContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

//TODO: impl contract orchestration ft_trasfer_call 
near_contract_standards::impl_fungible_token_core!(SteadyStudyTokenContract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(SteadyStudyTokenContract, token, on_account_closed);

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};
    use super::*;
    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

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
        let _contract = SteadyStudyTokenContract::default();
    }
    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = SteadyStudyTokenContract::new_default_meta(accounts(1), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    fn test_ft_report_study_commit() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = SteadyStudyTokenContract::new_default_meta(accounts(0), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());

        let data: Vec<String> = vec!["ipfs://hoge".to_string(), "ipfs://fuga".to_string(), "ipfs://poyo".to_string()];
        contract.ft_report_study_commit( accounts(1), None, &data);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());

        assert_eq!(contract.ft_balance_of(accounts(0)).0, (TOTAL_SUPPLY - 1));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, 1);
    }
}   