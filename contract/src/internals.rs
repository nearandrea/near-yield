use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, Promise, PromiseError};

impl Contract{
    pub(crate) fn update_user_balance(&mut self, account_id: AccountId, token_id: AccountId, amount: U128) {
        if let Some(mut user_balance) = self.user_balance.get(&account_id.clone()) {
            if let Some(mut current_amount) = user_balance.tokens.get(&(token_id.clone().to_string())) {
                current_amount += u128::from(amount);
                user_balance.tokens.insert(&(token_id.clone().to_string()), &current_amount);
            } else {
                user_balance.tokens.insert(&(token_id.clone().to_string()), &u128::from(amount));
            }
            self.user_balance.insert(&account_id.clone(), &user_balance);
        } else {
            let mut new_tokens = UnorderedMap::new(StorageKey::Tokens { account_id: account_id.clone()});
            new_tokens.insert(&(token_id.clone().to_string()), &u128::from(amount));
            self.user_balance.insert(&account_id.clone(), &UserBalance {tokens: new_tokens});
        }
    }

    pub(crate) fn withdraw_user_balance(&mut self, account_id: AccountId, token_id: AccountId, amount: U128) {
        if let Some(mut user_balance) = self.user_balance.get(&account_id.clone()) {
            if let Some(mut current_amount) = user_balance.tokens.get(&(token_id.clone().to_string())) {
                assert!(u128::from(amount) <= current_amount,"{}", ERR200_NOT_ENOUGH);
                current_amount = current_amount - u128::from(amount);
                user_balance.tokens.insert(&(token_id.clone().to_string()), &current_amount);
                self.user_balance.insert(&account_id.clone(), &user_balance);
            } else {
                env::panic_str(ERR200_NOT_ENOUGH);
            }
        } else {
            env::panic_str(ERR200_NOT_ENOUGH);
        }
    }

    pub(crate) fn transfer_pool_to_farm(&mut self, account_id: AccountId, pool_id: String, amount: U128) {
        // Check pool balance
        self.assert_pool_balance(account_id.clone(), pool_id.clone(), amount.clone());

        // Check if have no farm
        if self.farmers.get(&account_id.clone()).is_none() {
            let new_farms = UnorderedMap::new(StorageKey::Farms { account_id: account_id.clone()});
            self.farmers.insert(&account_id.clone(), &FarmData {farms: new_farms});
        }

        // Remove shares in pool
        let pool_amount = self.poolers.get(&account_id.clone()).unwrap().pools.get(&pool_id).unwrap_or(0) - u128::from(amount);
        self.poolers.get(&account_id.clone()).unwrap().pools.insert(&pool_id, &pool_amount);

        // Add LP in farm
        let mut farm_data = self.farmers.get(&account_id.clone()).unwrap();
        farm_data.farms.insert(&pool_id.clone(), &(farm_data.farms.get(&pool_id.clone()).unwrap_or(0) + u128::from(amount)));
        self.farmers.insert(&account_id.clone(), &farm_data);
    }

    pub(crate) fn assert_pool_balance(&self, account_id: AccountId, pool_id: String, amount: U128) {
        assert!(self.poolers.get(&account_id.clone()).is_some(), "{}", ERR200_NOT_ENOUGH);
        assert!(self.poolers.get(&account_id.clone()).unwrap().pools.get(&pool_id).unwrap_or(0) >= u128::from(amount), "{}", ERR200_NOT_ENOUGH);
    }

    pub(crate) fn parse_pool_id(&self, pool_id_str: String) -> String {
        assert!(pool_id_str.starts_with(":"), "{}", ERR300_ILLEGAL_TOKEN_ID);
        pool_id_str[1..pool_id_str.len()].to_string()
    }
}