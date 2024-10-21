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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_system_start() {
        let system_start = SystemStart::new("initializer".to_string());
        assert_eq!(system_start.initializer(), "initializer");
    }

    #[test]
    fn test_only_initializer() {
        let system_start = SystemStart::new("initializer".to_string());
        assert!(system_start.only_initializer(&"initializer".to_string()));
        assert!(!system_start.only_initializer(&"not_initializer".to_string()));
    }

    #[test]
    fn test_serialization_deserialization() {
        let system_start = SystemStart::new("initializer".to_string());
        let serialized = system_start.try_to_vec().unwrap();
        let deserialized = SystemStart::try_from_slice(&serialized).unwrap();
        
        assert_eq!(system_start.initializer(), deserialized.initializer());
    }
}