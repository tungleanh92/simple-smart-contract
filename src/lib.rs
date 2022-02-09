use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, Promise};
use near_sdk::collections::LookupMap;
use near_sdk::AccountId;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SmartContract {
    log_storage: LookupMap<String, Vec<String>>
}

impl Default for SmartContract {
    fn default() -> Self {
        Self {
            log_storage: LookupMap::new(b"a".to_vec())
        }
    }
}

#[near_bindgen]
impl SmartContract {
    pub fn add_log(&mut self, text: String, price: String) {
        let account_id=env::signer_account_id();
        if self.log_storage.contains_key(&account_id) {
            let mut tmp_list = match self.log_storage.get(&account_id) {
                Some(x) => x,
                None => vec![]
            };
            tmp_list.push(text+" || "+ &price+" NEAR");
            self.log_storage.insert(&account_id, &tmp_list);
        } else {
            let new_vec = vec![text+" || "+ &price+" NEAR"];
            self.log_storage.insert(&account_id, &new_vec);
        }
    }

    pub fn transfer_money(&mut self, account_id: AccountId, amount: f64) {
        Promise::new(account_id).transfer(amount as u128);
    }

    pub fn get_logs(self, user: String) -> Vec<String> {
        match self.log_storage.get(&user) {
            Some(x) => x,
            None => vec![]
        }
    }
}