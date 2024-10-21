#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        #[no_mangle]
        pub extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) =
                unsafe { $crate::entrypoint::deserialize(input) };

            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => 0,
                Err(e) => {
                    $crate::msg!("Error: {:?}", e);
                    1
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entrypoint_macro() {
        // This is more of a compilation test
        // If this compiles, the macro is working as expected
        entrypoint!(dummy_process_instruction);

        fn dummy_process_instruction(_program_id: &[u8], _accounts: &[u8], _instruction_data: &[u8]) -> Result<(), String> {
            Ok(())
        }

        // You can't directly call the generated entrypoint function in tests,
        // but you can verify that it compiles correctly
    }
}