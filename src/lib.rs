#[macro_use]
mod entrypoint_macros;
mod vault;
mod handlers;
mod dao;
mod core;
mod staking;

use borsh::{BorshDeserialize};

entrypoint!(process_instructions);

fn process_instructions(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { crate::entrypoint::deserialize(input) };

    let instruction: core::instructions::ContractInstruction = match ContractInstruction::try_from_slice(&instruction_data) {
        Ok(instr) => instr,
        Err(_) => return 1, // Error code for deserialization failure
    };

    match crate::core::handler::process_instructions(&program_id, &accounts, &instruction) {
        Ok(()) => 0,
        Err(e) => {
            crate::msg!("Error: {:?}", e);
            1 // Error code for processing failure
        }
    }
}

