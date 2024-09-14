#[macro_use]
mod entrypoint_macros;
mod vault;
mod handlers;
mod dao;
mod core;
mod staking;

use borsh::{BorshDeserialize, BorshSerialize};
use crate::fee_receiver::FeeReceiver;

mod fee_receiver;
mod models;
mod utils;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EntryPoint;

impl EntryPoint {
    pub fn process_instruction(instruction_data: &[u8], utxos: &[UtxoInfo]) -> Result<(), String> {
        // Deserialize instruction data to determine the action
        let action = utils::deserialize_action(instruction_data)?;

        match action {
            Action::Transfer { token_id, receiver, amount } => {
                FeeReceiver::transfer_token(token_id, receiver, amount, utxos)
            },
            Action::Approve { token_id, spender, amount } => {
                FeeReceiver::set_token_approval(token_id, spender, amount, utxos)
            },
            _ => Err("Unsupported action".to_string()),
        }
    }
}

