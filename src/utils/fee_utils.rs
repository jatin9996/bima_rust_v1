use borsh::{BorshDeserialize, BorshSerialize};

pub fn deserialize_action(data: &[u8]) -> Result<Action, String> {
    Action::try_from_slice(data).map_err(|_| "Failed to deserialize action".to_string())
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Action {
    Transfer { token_id: String, receiver: String, amount: u64 },
    Approve { token_id: String, spender: String, amount: u64 },
}
