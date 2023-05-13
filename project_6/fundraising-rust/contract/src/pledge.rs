use crate::Contract; // crate like a package import
use crate::ContractExt; // ContractExt -> instead lib.rs file (aka contract)

use near_sdk::{ AccountId, near_bindgen, env, Balance, Promise };
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

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
            total_amount: U128(self.pledges.get(&account_id).unwrap_or(0)),
        }
    }

    #[payable]
    pub fn pledge(&mut self) -> U128 {
        let pledger: AccountId = env::predecessor_account_id();
        let pledge_amount: Balance = env::attached_deposit();

        let mut pledge_so_far = self.pledges.get(&pledger).unwrap_or(0);

        let to_transfer: Balance = if pledge_so_far == 0 {
            assert!(pledge_amount > STORAGE_COST, "Attached at least {} yoctoNEAR", STORAGE_COST);

            pledge_amount - STORAGE_COST
        } else {
            pledge_amount
        };

        pledge_so_far += pledge_amount;

        self.pledges.insert(&pledger, &pledge_so_far);

        Promise::new(self.fundraiser.clone()).transfer(to_transfer);

        U128(pledge_so_far)
    }
}