pub trait BabelCore {  
    // Event Emitters 
    fn emit_fee_receiver_set(&self, fee_receiver: &str);
    fn emit_guardian_set(&self, guardian: &str);
    fn emit_new_owner_accepted(&self, old_owner: &str, owner: &str);
    fn emit_new_owner_committed(&self, owner: &str, pending_owner: &str, deadline: u128);
    fn emit_new_owner_revoked(&self, owner: &str, revoked_owner: &str);
    fn emit_paused(&self);
    fn emit_price_feed_set(&self, price_feed: &str);
    fn emit_unpaused(&self);

    // Function signatures
    fn accept_transfer_ownership(&mut self);

    fn commit_transfer_ownership(&mut self, new_owner: &str);

    fn revoke_transfer_ownership(&mut self);

    fn set_fee_receiver(&mut self, fee_receiver: &str);

    fn set_guardian(&mut self, guardian: &str);

    fn set_paused(&mut self, paused: bool);

    fn set_price_feed(&mut self, price_feed: &str);

    // UTXO and ZKVM specific methods
    fn add_liquidity(&mut self, utxo: Utxo);
    fn remove_liquidity(&mut self, utxo: Utxo);
    fn process_instruction(&self, instruction_data: &[u8]) -> Result<()>;

    // Getter functions
    fn ownership_transfer_delay(&self) -> u128;

    fn fee_receiver(&self) -> &str;

    fn guardian(&self) -> &str;

    fn owner(&self) -> &str;

    fn ownership_transfer_deadline(&self) -> u128;

    fn paused(&self) -> bool;

    fn pending_owner(&self) -> &str;

    fn price_feed(&self) -> &str;

    fn start_time(&self) -> u128;
}
