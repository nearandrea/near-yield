use crate::*;
use near_sdk::{env, log, near_bindgen, Promise, PromiseError,ext_contract};
use near_sdk::json_types::U128;

#[ext_contract(ext_ref_finance)]
trait RefFinance {
    fn add_liquidity(&self, pool_id: u64, amounts: Vec<U128>) -> U128;
    fn remove_liquidity(&self, min_amounts: Vec<U128>, pool_id: u64, shares: U128) -> Vec<U128>;
    fn withdraw(&self, token_id: AccountId, amount: U128, unregister: bool) -> U128;
}

#[ext_contract(ext_handle_token)]
trait HandleToken {
    fn ft_transfer_call(&self, receiver_id: AccountId, amount: U128, msg: &str) -> U128;
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128) -> U128;
}

#[near_bindgen]
impl Contract {
    pub fn call_add_liquidity(&mut self, pool_id: u64, amounts: Vec<U128>, token1: String, token2: String) -> PromiseOrValue<Vec<U128>> {
        if let Some(user_balance) = self.user_balance.get(&env::predecessor_account_id().clone()) {
            if user_balance.tokens.get(&token1).unwrap_or(0) >= u128::from(amounts[0]) && user_balance.tokens.get(&token2).unwrap_or(0) >= u128::from(amounts[1]) {
                let p1 = ext_handle_token::ext(token1.parse().unwrap())
                    .with_attached_deposit(DEPOSIT_YOCTO)
                    .with_static_gas(FT_CALL_GAS)
                    .ft_transfer_call(REF_CONTRACT.parse().unwrap(),amounts[0], "");
                
                let p2 = ext_handle_token::ext(token2.parse().unwrap())
                    .with_attached_deposit(DEPOSIT_YOCTO)
                    .with_static_gas(FT_CALL_GAS)
                    .ft_transfer_call(REF_CONTRACT.parse().unwrap(),amounts[1], "");
                
                let p3 = ext_ref_finance::ext(REF_CONTRACT.parse().unwrap())
                    .with_attached_deposit(STORAGE_COST)
                    .add_liquidity(pool_id, amounts.clone());
                
                let p4 = Self::ext(env::current_account_id())
                    .add_liquidity_callback(env::predecessor_account_id().clone(), pool_id.to_string(), amounts.clone(), token1.clone(),token2.clone());

                p1.then(p2).then(p3).then(p4);
            } else {
                env::panic_str(ERR200_NOT_ENOUGH);    
            }
        } else {
            env::panic_str(ERR200_NOT_ENOUGH);
        }
        PromiseOrValue::Value(amounts)
    }

    pub fn call_remove_liquidity(&mut self, min_amounts: Vec<U128>, pool_id: u64, shares: U128, token1: String, token2: String) -> PromiseOrValue<U128> {
        if let Some(farm_data) = self.farmers.get(&env::predecessor_account_id()) {
            if let Some(current_shares) = farm_data.pools.get(&pool_id.to_string()) {
                if current_shares >= u128::from(shares) {
                    let p1 = ext_ref_finance::ext(REF_CONTRACT.parse().unwrap())
                        .with_attached_deposit(DEPOSIT_YOCTO)
                        .remove_liquidity(min_amounts, pool_id, shares);
                    
                    let p1_1 = Self::ext(env::current_account_id())
                        .remove_liquidity_callback(env::predecessor_account_id().clone(), pool_id.to_string(), shares);

                    let p2 = ext_ref_finance::ext(REF_CONTRACT.parse().unwrap())
                        .with_static_gas(FT_CALL_WITHDRAW_TGAS)
                        .with_attached_deposit(DEPOSIT_YOCTO)
                        .withdraw(token1.parse().unwrap(), U128(0), false);
                    
                    let p2_1 = Self::ext(env::current_account_id())
                        .withdraw_callback(env::predecessor_account_id().clone(), token1);
                    
                    let p3 = ext_ref_finance::ext(REF_CONTRACT.parse().unwrap())
                        .with_static_gas(FT_CALL_WITHDRAW_TGAS)
                        .with_attached_deposit(DEPOSIT_YOCTO)
                        .withdraw(token2.parse().unwrap(), U128(0), false);
                    
                    let p3_1 = Self::ext(env::current_account_id())
                        .withdraw_callback(env::predecessor_account_id().clone(), token2);

                    (p1.then(p1_1)).and(p2.then(p2_1)).and(p3.then(p3_1));
                } else {
                    env::panic_str(ERR200_NOT_ENOUGH);
                }
            } else {
                env::panic_str(ERR200_NOT_ENOUGH);
            }
        } else {
            env::panic_str(ERR200_NOT_ENOUGH);
        }
        PromiseOrValue::Value(shares)
    }
    
    #[private]
    pub fn add_liquidity_callback(
        &mut self,
        account_id: AccountId,
        pool_id: String,
        amounts: Vec<U128>,
        token1: String,
        token2: String,
        #[callback_result] last_result: Result<String, PromiseError>,
    ) -> String {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!("The last result is {}", result);
            if let Some(mut user_balance) = self.user_balance.get(&account_id.clone()) {
                // add farm
                self.add_farmer(account_id, pool_id, U128(result.parse::<u128>().unwrap()));
                
                // modify user balance
                if user_balance.tokens.get(&token1).unwrap_or(0) >= u128::from(amounts[0]) && user_balance.tokens.get(&token2).unwrap_or(0) >= u128::from(amounts[1]) {
                    user_balance.tokens.insert(&token1, &(user_balance.tokens.get(&token1).unwrap_or(0) - u128::from(amounts[0])));
                    user_balance.tokens.insert(&token2, &(user_balance.tokens.get(&token2).unwrap_or(0) - u128::from(amounts[1])));
                    self.user_balance.insert(&env::predecessor_account_id().clone(), &user_balance);
                } else {
                    env::panic_str(ERR200_NOT_ENOUGH);
                }
                "OK".to_string()
            } else {
                "Update data failed".to_string()
            }
        } else {
            log!("The batch call failed and all calls got reverted");
            "".to_string()
        }
    }

    #[private]
    pub fn remove_liquidity_callback(
        &mut self,
        account_id: AccountId,
        pool_id: String,
        shares: U128,
        #[callback_result] last_result: Result<Vec<U128>, PromiseError>,
    ) -> String {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!("{} remove {} for pool {}", account_id.clone().to_string(), u128::from(shares).to_string() , pool_id);
            self.remove_farmer(account_id, pool_id, shares);
            "OK".to_string()
        } else {
            log!("The batch call failed and all calls got reverted");
            "".to_string()
        }
    }
    
    #[private]
    pub fn withdraw_callback(
        &mut self, 
        account_id: AccountId, 
        token_id: String,
        #[callback_result] last_result: Result<U128, PromiseError>,
    ) -> String {
        if let Ok(result) = last_result {
            self.update_user_balance(account_id.clone(), token_id.clone().parse().unwrap() , result);
            "OK".to_string()
        } else {
            log!("The batch call failed and all calls got reverted");
            "".to_string()
        }
    }

    fn add_farmer(&mut self, account_id: AccountId, pool_id: String, amount: U128) -> U128{
        log!("Add farmer: {} ,Pool Id: {}", account_id.clone().to_string(), pool_id);
        if let Some(mut farm_data) = self.farmers.get(&account_id.clone()) {
            log!("Farm data existed!");
            let mut pools = farm_data.pools;
            if let Some(mut current_amount) = pools.get(&pool_id) {
                current_amount += u128::from(amount);
                pools.insert(&pool_id, &current_amount);
            } else {
                pools.insert(&pool_id, &u128::from(amount));
            }
            farm_data.pools = pools;
            self.farmers.insert(&account_id.clone(), &farm_data);
        } else {
            log!("Create new farm data!");
            let mut new_pools = UnorderedMap::new(StorageKey::Pools { account_id: account_id.clone()});
            new_pools.insert(&pool_id, &u128::from(amount));
            self.farmers.insert(&account_id.clone(), &FarmData {pools: new_pools});
        }
        return amount;
    }

    fn remove_farmer(&mut self, account_id: AccountId, pool_id: String, amount: U128) -> U128{
        log!("Remove farmer: {} ,Pool Id: {}", account_id.clone().to_string(), pool_id);
        if let Some(mut farm_data) = self.farmers.get(&account_id.clone()) {
            if let Some(mut current_amount) = farm_data.pools.get(&pool_id) {
                assert!(u128::from(amount) <= current_amount,"{}", ERR200_NOT_ENOUGH);
                current_amount = current_amount - u128::from(amount);
                farm_data.pools.insert(&pool_id, &current_amount);
                self.farmers.insert(&account_id.clone(), &farm_data);
            } else {
                env::panic_str(ERR200_NOT_ENOUGH);
            }
        } else {
            env::panic_str(ERR200_NOT_ENOUGH);
        }
        amount
    }
}