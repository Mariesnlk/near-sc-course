use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ near_bindgen, env };
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Message {
    content: String,
    owner_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            // new empty
            messages: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Chat {
    pub fn get_total_messages(&self) -> usize {
        return self.messages.len();
    }

    pub fn add_message(&mut self, message_content: String) {
        let account_id = env::signer_account_id();
        let message_object = Message {
            content: message_content,
            owner_id: account_id.to_string()
        };
        self.messages.push(message_object);
    }

    pub fn get_messages(&mut self) -> Vec<Message> {
        self.messages.clone()
    }
}