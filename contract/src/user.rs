use crate::*;
use near_sdk::{env, log, near_bindgen, ext_contract};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserBalanceDTO {
    token_id: TokenId,
    amount: String
}

#[ext_contract(ext_deposit)]
trait TransferToken {
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128) -> U128;
}

#[near_bindgen]
impl Contract {
    pub fn get_balance(&self, account_id: AccountId) -> Vec<UserBalanceDTO>{
        let mut result = Vec::new();
        if let Some(user_balance) = self.user_balance.get(&account_id) {
            for (token_id, amount) in user_balance.tokens.iter() {
                result.push(UserBalanceDTO{
                    token_id: token_id,
                    amount: amount.to_string()
                });
            }
        }
        return result;
    }

    pub fn withdraw_balance(&mut self, token_id: AccountId, amount: U128) -> U128 {
        self.withdraw_user_balance(env::predecessor_account_id().clone(), token_id.clone(), amount);
        ext_deposit::ext(token_id.clone())
                .with_attached_deposit(DEPOSIT_YOCTO)
                .with_static_gas(FT_CALL_GAS)
                .ft_transfer(env::predecessor_account_id().clone(), amount);
        return amount;
    }

    pub fn withdraw_max(&mut self, token_id: AccountId) -> U128 {
        if let Some(mut user_balance) = self.user_balance.get(&env::predecessor_account_id().clone()) {
            let mut tokens = user_balance.tokens;
            let mut amount;
            if let Some(current_amount) = tokens.get(&(token_id.clone().to_string())) {
                amount = current_amount;
                tokens.insert(&(token_id.clone().to_string()), &0);
            } else {
                env::panic_str(ERR200_NOT_ENOUGH);
            }
            user_balance.tokens = tokens;
            self.user_balance.insert(&env::predecessor_account_id().clone(), &user_balance);
            ext_deposit::ext(token_id.clone())
                .with_attached_deposit(DEPOSIT_YOCTO)
                .with_static_gas(FT_CALL_GAS)
                .ft_transfer(env::predecessor_account_id().clone(), U128(amount));
            
            U128(amount)
        } else {
            env::panic_str(ERR200_NOT_ENOUGH);
        }
    }
}