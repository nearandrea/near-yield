// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, env, Promise, Balance, PromiseOrValue, BorshStorageKey};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::serde::Serialize;

use crate::utils::*;
use crate::errors::*;


// Import modules
mod utils;
mod errors;
mod owner;
mod stake;
mod token_receiver;
mod ref_finance;
mod farm;
mod user;
mod internals;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Stakes,
    Poolers,
    Farmers,
    UserBalance,
    Pools {account_id: AccountId},
    Tokens {account_id: AccountId},
    Farms {account_id: AccountId},
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FarmData {
    #[serde(skip_serializing)]
    pub farms: UnorderedMap<FarmId, Amount>
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolData {
    #[serde(skip_serializing)]
    pub pools: UnorderedMap<PoolId, Amount>
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserBalance {
    #[serde(skip_serializing)]
    pub tokens: UnorderedMap<TokenId, Amount>
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub message: String,
    pub owner_id: AccountId,
    pub stake_list: UnorderedMap<String, u128>,
    pub poolers: UnorderedMap<AccountId, PoolData>,
    pub farmers: UnorderedMap<AccountId, FarmData>,
    pub user_balance: UnorderedMap<AccountId, UserBalance>,
}

impl Default for Contract {
    fn default() -> Self {
        Self{
            message: DEFAULT_MESSAGE.to_string(),
            owner_id: DEFAULT_OWNER.parse().unwrap(),
            stake_list: UnorderedMap::new(StorageKey::Stakes),
            poolers: UnorderedMap::new(StorageKey::Poolers),
            farmers: UnorderedMap::new(StorageKey::Farmers),
            user_balance: UnorderedMap::new(StorageKey::UserBalance),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(message: String, owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            message: message.to_string(),
            stake_list: UnorderedMap::new(StorageKey::Stakes),
            poolers: UnorderedMap::new(StorageKey::Poolers),
            farmers: UnorderedMap::new(StorageKey::Farmers),
            user_balance: UnorderedMap::new(StorageKey::UserBalance),
        }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        log!("Saving greeting {}", message);
        self.message = message;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
