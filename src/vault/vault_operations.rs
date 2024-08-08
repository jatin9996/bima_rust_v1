use crate::vault::stablecoin_vault::VaultState;

pub struct VaultOperations {
    vault: VaultState,
}

impl VaultOperations {
    pub fn new() -> Self {
        Self {
            vault: VaultState::new(),
        }
    }

    pub fn deposit_bitcoin_utxo(&mut self, utxo_id: String, value: u64) {
        self.vault.deposit_bitcoin_utxo(utxo_id, value);
        println!("Deposited UTXO with value: {} bima", value);
    }

    pub fn issue_stablecoin(&mut self, btc_amount: u64) {
        self.vault.issue_stablecoin(btc_amount);
        println!("Issued {} stablecoins", btc_amount * self.vault.exchange_rate);
    }
}