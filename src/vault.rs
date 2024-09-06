use std::collections::HashMap;

pub struct VaultState {
    bitcoin_utxos: HashMap<String, u64>, // UTXOs identified by a unique ID and their value in satoshis
    bitcoin_balance: u64,
    stablecoin_supply: u64,
    exchange_rate: u64, // This will now be updated dynamically
}

impl VaultState {
    pub fn new() -> Self {
        Self {
            bitcoin_utxos: HashMap::new(),
            bitcoin_balance: 0,
            stablecoin_supply: 0,
            exchange_rate: 50000, // Assuming 1 BTC = 50,000 USD for the sake of 
        }
    }

    pub fn deposit_bitcoin_utxo(&mut self, utxo_id: String, value: u64) {
        self.bitcoin_utxos.insert(utxo_id, value);
        self.bitcoin_balance += value;
    }

    pub fn issue_stablecoin(&mut self, btc_amount: u64) {
        let stablecoins = btc_amount * self.exchange_rate;
        self.stablecoin_supply += stablecoins;
    }
}
