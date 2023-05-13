use crate::Contract; // crate like a package import
use crate::ContractExt; // ContractExt -> instead lib.rs file (aka contract)

use near_sdk::{ AccountId, near_bindgen };
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

#[near_bindgen] // treat like a smart contract
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pledge {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    pub fn get_number_of_pledges(&self) -> u64 {
        self.pledges.len()
    }

    pub fn get_pledge_for_account(&self, account_id: AccountId) -> Pledge {
        Pledge {
            account_id: account_id.clone(),
            total_amount: U128(self.pledges.get(&account_id).unwrap_or(0))
        }
    }
}