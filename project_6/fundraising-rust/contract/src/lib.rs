// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ near_bindgen, AccountId, env };
use near_sdk::collections::{ UnorderedMap };

mod pledge;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fundraiser: AccountId,
    pub pledges: UnorderedMap<AccountId, u128>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            fundraiser: "mariesnlktestnet.testnet".parse().unwrap(),
            pledges: UnorderedMap::new(b"p"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    #[private] // possible to call only by owner
    pub fn init(fundraiser_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Contract already made");

        Self {
            fundraiser: fundraiser_id,
            pledges: UnorderedMap::new(b"p"),
        }
    }

    pub fn get_fundraiser(&self) -> AccountId {
        self.fundraiser.clone()
    }

    #[private] // possible to call only by owner
    pub fn set_fundraiser(&mut self, new_fundraiser: AccountId) {
        self.fundraiser = new_fundraiser;
    }
}