use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    // Handle write data when token received
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        log!("Sender: {}, Token: {}, Msg: {}",sender_id.clone(), env::predecessor_account_id().clone(), msg.clone());
        
        // Update user balance
        self.update_user_balance(sender_id.clone(),env::predecessor_account_id().clone(),amount);
        PromiseOrValue::Value(U128(0))
    }   
}