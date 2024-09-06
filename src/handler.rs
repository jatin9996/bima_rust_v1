use crate::vault::VaultState;

pub fn process_instructions(instructions: Vec<Instruction>) -> Result<(), u64> {
    let mut vault = VaultState::new();
    for instruction in instructions {
        match instruction {
            Instruction::IssueStablecoin { btc_amount } => {
                vault.issue_stablecoin(btc_amount);
            },
            _ => return Err(2), // Unsupported instruction
        }
    }
    Ok(())
}