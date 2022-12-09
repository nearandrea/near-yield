use crate::*;
use near_sdk::{env, log, near_bindgen};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FarmDTO {
    pool_id: PoolId,
    amount: String
}


#[near_bindgen]
impl Contract {
    pub fn get_farms(&self,  account_id: AccountId) -> Vec<FarmDTO> {
        let mut farms = Vec::new();
        if let Some(farm_data) = self.farmers.get(&account_id) {
            for (pool_id, amount) in farm_data.pools.iter() {
                farms.push(FarmDTO { pool_id: pool_id, amount: amount.to_string()});
            }
            return farms;
        } else {
            return farms;
        }
    }

    pub fn modify_farm(&mut self, account_id: AccountId, pool_id: String, shares: U128) -> U128 {
        self.assert_owner();
        if let Some(mut farm_data) = self.farmers.get(&account_id) {
            farm_data.pools.insert(&pool_id, &u128::from(shares));
            self.farmers.insert(&account_id, &farm_data);
            shares
        } else {
            U128(0)
        }
    }

    pub fn get_farm(&self, pool_id: String, account_id: AccountId) -> U128 {
        if let Some(farm_data) = self.farmers.get(&account_id) {
            if let Some(amount) = farm_data.pools.get(&pool_id) {
                return U128(amount);
            } else {
                return U128(0);
            }
        } else {
            return U128(0);
        }
    }
}