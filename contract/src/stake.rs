use crate::*;
use near_sdk::serde::Serialize;
use near_sdk::json_types::U128;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Stake {
    pub key: String, 
    pub amount: U128,
}

#[near_bindgen]
impl Contract {

    // Stake $NEAR by number of deposit
    #[payable]
    pub fn stake(&mut self) -> U128 {
        let staker: AccountId = env::predecessor_account_id();
        let mut stake_amount: Balance = env::attached_deposit();
        let mut amount =  self.stake_list.get(&(staker.clone().to_string() + "_near")).unwrap_or(0);

        log!("stake_amount: {}",stake_amount);
        if amount == 0 {
            assert!(stake_amount > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);
            stake_amount = stake_amount - STORAGE_COST;
        }

        amount += stake_amount;
        self.stake_list.insert(&(staker.clone().to_string() + "_near"), &amount);
        return U128(stake_amount);
    }

    // Unstake amount of specific $NEAR amount
    pub fn unstake(&mut self, unstake_amount: Option<U128>) -> U128 {
        let staker: AccountId = env::predecessor_account_id();
        let mut amount =  self.stake_list.get(&(staker.clone().to_string() + "_near")).unwrap_or(0);
        if amount != 0 {
            if unstake_amount.is_some() {
                if amount >= u128::from(unstake_amount.unwrap()) {
                    amount = amount - u128::from(unstake_amount.unwrap());
                    Promise::new(staker.clone()).transfer(u128::from(unstake_amount.unwrap()));
                    self.stake_list.insert(&(staker.clone().to_string() + "_near"), &amount);
                    return unstake_amount.unwrap();
                } else {
                    env::panic_str(ERR200_NOT_ENOUGH);
                }
            } else {
                Promise::new(staker.clone()).transfer(amount);
                self.stake_list.insert(&(staker.clone().to_string() + "_near"), &ZERO);
                
            }
        }
        return U128(amount);
    }

    // Get $NEAR stake for specific account
    pub fn get_stake_for_account(&self, account_id: AccountId) -> Stake {
        Stake {
            key: account_id.clone().to_string() + "_near",
            amount: U128(self.stake_list.get(&(account_id.clone().to_string() + "_near")).unwrap_or(0))
        }
    }

    // Get all stake account
    pub fn get_stakes(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Stake> {
        self.assert_owner();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        //iterate through donation
        self.stake_list.keys()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            .map(|account| self.get_stake_for_account(account.replace("_near","").parse().unwrap()))
            //since we turned map into an iterator, we need to turn it back into a vector to return
            .collect()
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    const ATTACHED_DEPOSIT: u128 = 1_000_000_000_000_000_000_000_000;
    const ATTACHED_DEPOSIT2: u128 = 2_000_000_000_000_000_000_000_000;
    const UNSTAKE_BALANCE: u128 = 500_000_000_000_000_000_000_000;

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        testing_env!(context.attached_deposit(ATTACHED_DEPOSIT).build());
        let contract = Contract::new("".to_string(), accounts(0));
        return(context, contract);
    }

    #[test]
    fn stake_1() {
        let (_context, mut contract) = setup_contract();
        contract.stake();
        let stakes: Vec<Stake> = contract.get_stakes(None, None);
        assert_eq!(stakes.len(), 1);
    }

    #[test]
    fn stake_2() {
        let (mut context, mut contract) = setup_contract();
        contract.stake();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        testing_env!(context.attached_deposit(ATTACHED_DEPOSIT2).build());
        contract.stake();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let stakes: Vec<Stake> = contract.get_stakes(None, None);
        assert_eq!(stakes.len(), 2);
    }

    #[test]
    fn stake_3() {
        let (_context, mut contract) = setup_contract();
        contract.stake();
        let stakes: Vec<Stake> = contract.get_stakes(None, None);
        let key = accounts(0).clone().to_string() + "_near";
        for n in stakes.iter() {
            if n.key == key {
                assert_eq!(n.amount, U128(ATTACHED_DEPOSIT - STORAGE_COST));
            }
        }
    }

    #[test]
    fn unstake_1() {
        let (_context, mut contract) = setup_contract();
        contract.stake();
        let unstake_amount = contract.unstake(None);
        assert_eq!(unstake_amount, U128(ATTACHED_DEPOSIT - STORAGE_COST));
    }

    #[test]
    fn unstake_2() {
        let (_context, mut contract) = setup_contract();
        contract.stake();
        contract.unstake(Some(U128(UNSTAKE_BALANCE)));
        let stakes: Vec<Stake> = contract.get_stakes(None, None);
        let key = accounts(0).clone().to_string() + "_near";
        for n in stakes.iter() {
            if n.key == key {
                assert_eq!(n.amount, U128(ATTACHED_DEPOSIT - STORAGE_COST - UNSTAKE_BALANCE));
            }
        }
    }
}