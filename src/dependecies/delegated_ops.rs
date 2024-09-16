use std::collections::HashMap;

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


