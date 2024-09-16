use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BabelTokenData {
    // Add fields that you want to serialize/deserialize
    pub owner: String,
    pub spender: String,
    pub value: u128,
    // Add other fields as necessary
}

pub trait BabelToken {
    // Event Emitters (Represented as functions for simplicity)
    fn emit_approval(&self, owner: &str, spender: &str, value: u128);
    fn emit_message_failed(&self, src_chain_id: u16, src_address: &[u8], nonce: u64, payload: &[u8], reason: &[u8]);
    fn emit_ownership_transferred(&self, previous_owner: &str, new_owner: &str);
    fn emit_receive_from_chain(&self, src_chain_id: u16, to: &str, amount: u128);
    fn emit_retry_message_success(&self, src_chain_id: u16, src_address: &[u8], nonce: u64, payload_hash: [u8; 32]);
    fn emit_send_to_chain(&self, dst_chain_id: u16, from: &str, to_address: &[u8], amount: u128);
    fn emit_set_min_dst_gas(&self, dst_chain_id: u16, packet_type: u16, min_dst_gas: u128);
    fn emit_set_precrime(&self, precrime: &str);
    fn emit_set_trusted_remote(&self, remote_chain_id: u16, path: &[u8]);
    fn emit_set_trusted_remote_address(&self, remote_chain_id: u16, remote_address: &[u8]);
    fn emit_set_use_custom_adapter_params(&self, use_custom_adapter_params: bool);
    fn emit_transfer(&self, from: &str, to: &str, value: u128);

    // Function signatures
    fn approve(&mut self, spender: &str, amount: u128) -> bool;

    fn decrease_allowance(&mut self, spender: &str, subtracted_value: u128) -> bool;

    fn force_resume_receive(&mut self, src_chain_id: u16, src_address: &[u8]);

    fn increase_allowance(&mut self, spender: &str, added_value: u128) -> bool;

    fn lz_receive(&mut self, src_chain_id: u16, src_address: &[u8], nonce: u64, payload: &[u8]);

    fn mint_to_vault(&mut self, total_supply: u128) -> bool;

    fn nonblocking_lz_receive(&mut self, src_chain_id: u16, src_address: &[u8], nonce: u64, payload: &[u8]);

    fn permit(
        &mut self,
        owner: &str,
        spender: &str,
        amount: u128,
        deadline: u128,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    );

    fn renounce_ownership(&mut self);

    fn set_config(&mut self, version: u16, chain_id: u16, config_type: u128, config: &[u8]);

    fn set_min_dst_gas(&mut self, dst_chain_id: u16, packet_type: u16, min_gas: u128);

    fn set_payload_size_limit(&mut self, dst_chain_id: u16, size: u128);

    fn set_precrime(&mut self, precrime: &str);

    fn set_receive_version(&mut self, version: u16);

    fn set_send_version(&mut self, version: u16);

    fn set_trusted_remote(&mut self, src_chain_id: u16, path: &[u8]);

    fn set_trusted_remote_address(&mut self, remote_chain_id: u16, remote_address: &[u8]);

    fn set_use_custom_adapter_params(&mut self, use_custom_adapter_params: bool);

    fn transfer(&mut self, to: &str, amount: u128) -> bool;

    fn transfer_from(&mut self, from: &str, to: &str, amount: u128) -> bool;

    fn transfer_ownership(&mut self, new_owner: &str);

    fn transfer_to_locker(&mut self, sender: &str, amount: u128) -> bool;

    fn retry_message(
        &mut self,
        src_chain_id: u16,
        src_address: &[u8],
        nonce: u64,
        payload: &[u8],
    ) -> ();

    fn send_from(
        &mut self,
        from: &str,
        dst_chain_id: u16,
        to_address: &[u8],
        amount: u128,
        refund_address: &str,
        zro_payment_address: &str,
        adapter_params: &[u8],
    ) -> ();

    // Getter functions
    fn default_payload_size_limit(&self) -> u128;

    fn no_extra_gas(&self) -> u128;

    fn pt_send(&self) -> u16;

    fn allowance(&self, owner: &str, spender: &str) -> u128;

    fn balance_of(&self, account: &str) -> u128;

    fn circulating_supply(&self) -> u128;

    fn decimals(&self) -> u8;

    fn domain_separator(&self) -> [u8; 32];

    fn estimate_send_fee(
        &self,
        dst_chain_id: u16,
        to_address: &[u8],
        amount: u128,
        use_zro: bool,
        adapter_params: &[u8],
    ) -> (u128, u128);

    fn failed_messages(&self, src_chain_id: u16, src_address: &[u8], nonce: u64) -> [u8; 32];

    fn get_config(
        &self,
        version: u16,
        chain_id: u16,
        address: &str,
        config_type: u128,
    ) -> Vec<u8>;

    fn get_trusted_remote_address(&self, remote_chain_id: u16) -> Vec<u8>;

    fn is_trusted_remote(&self, src_chain_id: u16, src_address: &[u8]) -> bool;

    fn locker(&self) -> &str;

    fn lz_endpoint(&self) -> &str;

    fn max_total_supply(&self) -> u128;

    fn min_dst_gas_lookup(&self, dst_chain_id: u16, packet_type: u16) -> u128;

    fn name(&self) -> String;

    fn nonces(&self, owner: &str) -> u128;

    fn owner(&self) -> &str;

    fn payload_size_limit_lookup(&self, dst_chain_id: u16) -> u128;

    fn permit_type_hash(&self) -> [u8; 32];

    fn precrime(&self) -> &str;

    fn supports_interface(&self, interface_id: [u8; 4]) -> bool;

    fn symbol(&self) -> String;

    fn token(&self) -> &str;

    fn total_supply(&self) -> u128;

    fn trusted_remote_lookup(&self, chain_id: u16) -> Vec<u8>;

    fn use_custom_adapter_params(&self) -> bool;

    fn vault(&self) -> &str;

    fn version(&self) -> String;
}