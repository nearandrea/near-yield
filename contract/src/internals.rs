use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env};

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

        // Remove amoun in pool
        let pool_amount = self.poolers.get(&account_id.clone()).unwrap().pools.get(&pool_id).unwrap_or(0) - u128::from(amount);
        self.poolers.get(&account_id.clone()).unwrap().pools.insert(&pool_id, &pool_amount);

        // Add amoun in farm
        let mut farm_data = self.farmers.get(&account_id.clone()).unwrap();
        farm_data.farms.insert(&pool_id.clone(), &(farm_data.farms.get(&pool_id.clone()).unwrap_or(0) + u128::from(amount)));
        self.farmers.insert(&account_id.clone(), &farm_data);
    }

    pub(crate) fn transfer_farm_to_pool(&mut self, account_id: AccountId, farm_id: String, amount: U128) {
        // Check pool balance
        self.assert_farm_balance(account_id.clone(), farm_id.clone(), amount.clone());

        // Remove amount in farm
        let farm_amount = self.farmers.get(&account_id.clone()).unwrap().farms.get(&farm_id).unwrap_or(0) - u128::from(amount);
        self.farmers.get(&account_id.clone()).unwrap().farms.insert(&farm_id, &farm_amount);

        // Add amount in pool
        let mut pool_data = self.poolers.get(&account_id.clone()).unwrap();
        pool_data.pools.insert(&farm_id.clone(), &(pool_data.pools.get(&farm_id.clone()).unwrap_or(0) + u128::from(amount)));
        self.poolers.insert(&account_id.clone(), &pool_data);
    }

    pub(crate) fn assert_pool_balance(&self, account_id: AccountId, pool_id: String, amount: U128) {
        assert!(self.poolers.get(&account_id.clone()).is_some(), "{}", ERR202_NOT_ENOUGH_POOL);
        assert!(self.poolers.get(&account_id.clone()).unwrap().pools.get(&pool_id).unwrap_or(0) >= u128::from(amount), "{}", ERR202_NOT_ENOUGH_POOL);
    }

    pub(crate) fn assert_farm_balance(&self, account_id: AccountId, farm_id: String, amount: U128) {
        assert!(self.farmers.get(&account_id.clone()).is_some(), "{}", ERR201_NOT_ENOUGH_FARM);
        assert!(self.farmers.get(&account_id.clone()).unwrap().farms.get(&farm_id).unwrap_or(0) >= u128::from(amount), "{}", ERR201_NOT_ENOUGH_FARM);
    }

    pub(crate) fn parse_pool_id(&self, pool_id_str: String) -> String {
        assert!(pool_id_str.starts_with(":"), "{}", ERR300_ILLEGAL_TOKEN_ID);
        pool_id_str[1..pool_id_str.len()].to_string()
    }

    pub(crate) fn parse_pool_id_from_seed_id(&self, seed_id: String) -> String {
        assert!(seed_id.contains(POOL_SEPARATE), "{}", ERR301_ILLEGAL_SEED_ID);
        let v: Vec<&str> = seed_id.split(POOL_SEPARATE).collect();
        assert!(v.len() == 2, "{}", ERR301_ILLEGAL_SEED_ID);
        assert!(v[1] != "", "{}", ERR301_ILLEGAL_SEED_ID);
        v[1].to_string()
    }
}