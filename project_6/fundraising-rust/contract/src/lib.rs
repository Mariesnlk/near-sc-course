// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fundraiser: AccountId,
    pub pledges: UnorderedMap<AccountId, u128>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  
}
