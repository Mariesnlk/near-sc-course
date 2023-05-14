use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ near_bindgen, env, AccountId };
use near_sdk::collections::LookupMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    user_accounts: LookupMap<AccountId, u128>,
    total_supply: u128,
}

impl Default for FungibleToken {
    fn default() -> FungibleToken {
        let mut contract = FungibleToken {
            user_accounts: LookupMap::new(b'm'),
            total_supply: 100,
        };

        let account_id = env::signer_account_id();
        contract.user_accounts.insert(&account_id, &contract.total_supply);
        contract
    }
}

#[near_bindgen]
impl FungibleToken {
    pub fn get_total_supply(&self) -> u128 {
        return self.total_supply.clone();
    }

    pub fn get_balance_of(&self, account_id: AccountId) -> u128 {
        if let None = self.user_accounts.get(&account_id) {
            return 0;
        }
        return self.user_accounts.get(&account_id).unwrap(); // to avoid Option<>
    }

    pub fn transfer(&mut self, receiver_id: AccountId, tokens: u128) {
        let sender_id = env::signer_account_id();

        let initial_sender_amount;
        if let None = self.user_accounts.get(&sender_id) {
            initial_sender_amount = 0;
        } else {
            // LookupMap returns an Option
            initial_sender_amount = self.user_accounts.get(&sender_id).unwrap();
        }

        assert!(initial_sender_amount >= tokens, "Sender does not have enough tokens.");
        self.user_accounts.insert(&sender_id, &(initial_sender_amount - tokens));

        let initial_receiver_amount;
        if let None = self.user_accounts.get(&receiver_id) {
            initial_receiver_amount = 0;
        } else {
            initial_receiver_amount = self.user_accounts.get(&receiver_id).unwrap();
        }

        self.user_accounts.insert(&receiver_id, &(initial_sender_amount + tokens));
    }
}