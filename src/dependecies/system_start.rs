mod system_start {
    use std::time::{SystemTime, UNIX_EPOCH};
    use borsh::{BorshSerialize, BorshDeserialize};

    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct SystemStart {
        start_time: u64,
    }

    impl SystemStart {
        pub fn new(start_time: u64) -> Self {
            Self { start_time }
        }

        pub fn get_week(&self) -> u64 {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            (current_time - self.start_time) / (7 * 24 * 60 * 60)
        }
    }

    pub trait IBabelCore {
        fn start_time(&self) -> u64;
    }

    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct BabelCore {
        start_time: u64,
    }

    impl BabelCore {
        pub fn new(start_time: u64) -> Self {
            Self { start_time }
        }

        pub fn start_time(&self) -> u64 {
            self.start_time
        }
    }
}
