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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::instructions::ContractInstruction;

    #[test]
    fn test_process_instructions() {
        // This is a basic test and might need to be adjusted based on your actual implementation
        let mut input_data = Vec::new();
        // Simulate program_id, accounts, and instruction_data
        input_data.extend_from_slice(&[0u8; 32]); // program_id
        input_data.extend_from_slice(&[0u8; 32]); // accounts
        
        // Create a dummy ContractInstruction
        let dummy_instruction = ContractInstruction::Initialize;
        let instruction_data = dummy_instruction.try_to_vec().unwrap();
        input_data.extend_from_slice(&instruction_data);

        let result = process_instructions(input_data.as_mut_ptr());
        assert_eq!(result, 0); // Assuming 0 means success
    }
}
