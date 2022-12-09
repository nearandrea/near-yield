use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, Promise, PromiseError,ext_contract};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        log!("Sender: {}, Token: {}",sender_id.clone(), env::predecessor_account_id().clone());
        self.update_user_balance(sender_id.clone(),env::predecessor_account_id().clone(),amount);
        PromiseOrValue::Value(U128(0))
    }   
}