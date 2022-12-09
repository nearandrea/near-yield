use crate::*;

#[near_bindgen]
impl Contract {
    // Get owner id of this contract
    // Owner can do setting for some service fee, turn on/off service...
    pub fn get_owner_id(&self) -> AccountId {
        log!("Owner_id: {}", self.owner_id.clone());
        return self.owner_id.clone();
    }

    // Re-setting owner id
    pub fn set_owner_id(&mut self, owner_id: AccountId){
        log!("New Owner_id: {}", owner_id);
        self.assert_owner();
        self.owner_id = owner_id;
    }

    // Check owner, use as common
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "{}", ERR100_NOT_ALLOWED
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new("".to_string(), accounts(0));
        return(context, contract);
    }

    #[test]
    fn get_default_owner() {
        let contract = Contract::default();
        assert_eq!(
            contract.get_owner_id(),
            DEFAULT_OWNER.parse().unwrap()
        );
    }

    #[test]
    fn set_owner_id() {
        let (_context, mut contract) = setup_contract();
        contract.set_owner_id("andreapn1709.testnet".parse().unwrap());
        assert_eq!(
            contract.get_owner_id(),
            DEFAULT_OWNER.parse().unwrap()
        );
    }

}