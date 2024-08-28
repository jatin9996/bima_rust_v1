#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            use std::collections::HashMap;
            let (program_id, utxos, instruction_data) =
                unsafe { $crate::entrypoint::deserialize(input) };
            match $process_instruction(&program_id, &utxos, &instruction_data) {
                Ok(()) => {
                    return 0;
                }
                Err(e) => {
                    $crate::msg!("program return an error {:?}", e);
                    return 1;
                }
            }
        }
        $crate::custom_heap_default!();
        $crate::custom_panic_default!();
    };
}