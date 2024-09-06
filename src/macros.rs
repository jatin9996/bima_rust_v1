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