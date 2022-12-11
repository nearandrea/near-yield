use crate::*;
use near_sdk::{log, near_bindgen};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolDTO {
    pool_id: PoolId,
    amount: String
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FarmDTO {
    farm_id: FarmId,
    amount: String
}


#[near_bindgen]
impl Contract {
    // get current pools by user 
    pub fn get_pools(&self,  account_id: AccountId) -> Vec<PoolDTO> {
        let mut pools = Vec::new();
        if let Some(pool_data) = self.poolers.get(&account_id) {
            for (pool_id, amount) in pool_data.pools.iter() {
                pools.push(PoolDTO { pool_id: pool_id, amount: amount.to_string()});
            }
            return pools;
        } else {
            return pools;
        }
    }

    pub fn get_farms(&self,  account_id: AccountId) -> Vec<FarmDTO> {
        let mut farms = Vec::new();
        if let Some(farm_data) = self.farmers.get(&account_id) {
            for (farm_id, amount) in farm_data.farms.iter() {
                farms.push(FarmDTO { farm_id: farm_id, amount: amount.to_string()});
            }
            return farms;
        } else {
            return farms;
        }
    }

    pub fn modify_pool(&mut self, account_id: AccountId, pool_id: String, shares: U128) -> U128 {
        self.assert_owner();
        if let Some(mut pool_data) = self.poolers.get(&account_id) {
            pool_data.pools.insert(&pool_id, &u128::from(shares));
            self.poolers.insert(&account_id, &pool_data);
            shares
        } else {
            U128(0)
        }
    }

    pub fn modify_farm(&mut self, account_id: AccountId, farm_id: String, amount: U128) -> U128 {
        self.assert_owner();
        if let Some(mut farm_data) = self.farmers.get(&account_id) {
            log!("Modify {}, amount: {}", farm_id.clone(), u128::from(amount).to_string());
            farm_data.farms.insert(&farm_id, &u128::from(amount));
            self.farmers.insert(&account_id, &farm_data);
            amount
        } else {
            U128(0)
        }
    }

    pub fn get_pool(&self, pool_id: String, account_id: AccountId) -> U128 {
        if let Some(pool_data) = self.poolers.get(&account_id) {
            if let Some(amount) = pool_data.pools.get(&pool_id) {
                return U128(amount);
            } else {
                return U128(0);
            }
        } else {
            return U128(0);
        }
    }
}