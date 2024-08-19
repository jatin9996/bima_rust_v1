#[macro_export]
macro_rules! entrypoint {
    ($handler:path) => {
        use crate::instruction::Instruction; // Adjust the path as necessary

        #[no_mangle]
        pub extern "C" fn entrypoint(input: *const u8, length: usize) -> u64 {
            let data = unsafe { std::slice::from_raw_parts(input, length) };
            let instructions: Result<Vec<Instruction>, _> = borsh::BorshDeserialize::deserialize(&mut data.as_ref());
            match instructions {
                Ok(instructions) => {
                    match $handler(instructions) {
                        Ok(_) => 0, // Success
                        Err(e) => e as u64, // Error code
                    }
                },
                Err(_) => 1, // Deserialization failed
            }
        }
    };
}