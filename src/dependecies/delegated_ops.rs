use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DelegatedOps {
    is_approved_delegate: HashMap<(String, String), bool>,
}

impl DelegatedOps {
    pub fn new() -> Self {
        Self {
            is_approved_delegate: HashMap::new(),
        }
    }

    pub fn set_delegate_approval(&mut self, caller: String, delegate: String, is_approved: bool) {
        self.is_approved_delegate.insert((caller, delegate), is_approved);
    }

    pub fn is_approved_delegate(&self, owner: String, caller: String) -> bool {
        *self.is_approved_delegate.get(&(owner, caller)).unwrap_or(&false)
    }

    pub fn ensure_caller_or_delegated(&self, account: String, caller: String) {
        assert!(
            caller == account || self.is_approved_delegate(account.clone(), caller),
            "Delegate not approved"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_delegated_ops() {
        let delegated_ops = DelegatedOps::new("owner".to_string(), "operator".to_string());
        assert_eq!(delegated_ops.owner(), "owner");
        assert_eq!(delegated_ops.operator(), "operator");
    }

    #[test]
    fn test_only_operator() {
        let delegated_ops = DelegatedOps::new("owner".to_string(), "operator".to_string());
        assert!(delegated_ops.only_operator(&"operator".to_string()));
        assert!(!delegated_ops.only_operator(&"not_operator".to_string()));
    }

    #[test]
    fn test_serialization_deserialization() {
        let delegated_ops = DelegatedOps::new("owner".to_string(), "operator".to_string());
        let serialized = delegated_ops.try_to_vec().unwrap();
        let deserialized = DelegatedOps::try_from_slice(&serialized).unwrap();
        
        assert_eq!(delegated_ops.owner(), deserialized.owner());
        assert_eq!(delegated_ops.operator(), deserialized.operator());
    }
}
