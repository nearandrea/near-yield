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
}