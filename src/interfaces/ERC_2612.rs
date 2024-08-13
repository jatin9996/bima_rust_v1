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
