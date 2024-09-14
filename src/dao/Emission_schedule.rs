use borsh::{BorshSerialize, BorshDeserialize};
use crate::models::state::EmissionState;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EmissionScheduleContract;

impl EmissionScheduleContract {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_instruction(&self, input_data: &[u8], state_utxo_data: &[u8]) -> Vec<u8> {
        let mut state: EmissionState = borsh::deserialize(state_utxo_data).unwrap();
        // Process input data and update state
        borsh::serialize(&state).unwrap()
    }
}
