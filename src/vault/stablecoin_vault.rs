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
            exchange_rate: get_btc_to_usd_rate(), // Initialize with a dynamic rate
        }
    }

    pub fn deposit_bitcoin_utxo(&mut self, utxo_id: String, value: u64) {
        self.bitcoin_utxos.insert(utxo_id, value);
        self.bitcoin_balance += value;
    }

    pub fn issue_stablecoin(&mut self, btc_amount: u64) {
        // Update the exchange rate each time before issuing stablecoins
        self.exchange_rate = get_btc_to_usd_rate();
        let stablecoins = btc_amount * self.exchange_rate;
        self.stablecoin_supply += stablecoins;
    }
}

// Function to fetch the BTC to USD rate from an oracle
fn get_btc_to_usd_rate() -> u64 {
    
    50000 // Assuming 1 BTC = 50,000 USD for the sake of example
}

fn main() {
    let mut vault = VaultState::new();
    vault.deposit_bitcoin_utxo("utxo123".to_string(), 1000000); // Deposit 1 BTC
    let rate = get_btc_to_usd_rate(); // Get current BTC to USD rate
    vault.issue_stablecoin(1); // Issue stablecoin equivalent to 1 BTC

    println!("BTC Balance: {}", vault.bitcoin_balance);
    println!("Stablecoin Supply: {}", vault.stablecoin_supply);
}

use crate::vault::stablecoin_vault::VaultState;

fn manage_vault_operations() {
    let mut vault = VaultState::new();
    vault.deposit_bitcoin_utxo("utxo456".to_string(), 2000000);
    let rate = 50000; // This should ideally come from a real oracle
    vault.issue_stablecoin(5);
    println!("Updated BTC Balance: {}", vault.bitcoin_balance);
    println!("Stablecoins Issued: {}", vault.stablecoin_supply);
}