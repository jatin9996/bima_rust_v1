use borsh::{BorshDeserialize, BorshSerialize};

pub trait IERC2612 {
    /// Sets `amount` as the allowance of `spender` over `owner`'s tokens,
    /// given `owner`'s signed approval.
    ///
    /// # Parameters
    /// - `owner`: Address of the token owner.
    /// - `spender`: Address of the spender.
    /// - `amount`: Amount of tokens to approve.
    /// - `deadline`: Timestamp by which the permit must be used.
    /// - `v`, `r`, `s`: Signature components.
    ///
    /// # Returns
    /// Result indicating success or failure.
    fn permit(
        &self,
        owner: &str,
        spender: &str,
        amount: u128,
        deadline: u128,
        v: u8,
        r: [u8; 32],
        s: [u8; 32]
    ) -> Result<(), String>;

    /// Returns the current ERC2612 nonce for `owner`.
    ///
    /// # Parameters
    /// - `owner`: Address of the token owner.
    ///
    /// # Returns
    /// Current nonce for the owner.
    fn nonces(&self, owner: &str) -> u128;

    /// Returns the version of the ERC2612 implementation.
    ///
    /// # Returns
    /// Version string.
    fn version(&self) -> &str;

    /// Returns the permit type hash.
    ///
    /// # Returns
    /// Permit type hash.
    fn permit_type_hash(&self) -> [u8; 32];

    /// Returns the domain separator.
    ///
    /// # Returns
    /// Domain separator.
    fn domain_separator(&self) -> [u8; 32];
}

// struct implementing IERC2612 with Borsh serialization
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ERC2612Impl {
    // Define the fields required for your implementation
    // For example:
    pub owner: String,
    pub spender: String,
    pub amount: u128,
    pub deadline: u128,
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
    // Add other necessary fields
}

impl IERC2612 for ERC2612Impl {
    // Implement the trait methods
    fn permit(
        &self,
        owner: &str,
        spender: &str,
        amount: u128,
        deadline: u128,
        v: u8,
        r: [u8; 32],
        s: [u8; 32]
    ) -> Result<(), String> {
        // Implementation here
        Ok(())
    }

    fn nonces(&self, owner: &str) -> u128 {
        // Implementation here
        0
    }

    fn version(&self) -> &str {
        // Implementation here
        "1.0"
    }

    fn permit_type_hash(&self) -> [u8; 32] {
        // Implementation here
        [0; 32]
    }

    fn domain_separator(&self) -> [u8; 32] {
        // Implementation here
        [0; 32]
    }
}
