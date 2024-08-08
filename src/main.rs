mod vault {
    pub mod stablecoin_vault;
    pub mod vault_operations;
}

use crate::vault::vault_operations::VaultOperations;

fn main() {
    let mut vault_ops = VaultOperations::new();
    vault_ops.deposit_bitcoin_utxo("utxo123".to_string(), 1000000); // 1 BTC assuming 100 satoshis = 1 BTC
    vault_ops.issue_stablecoin(1); // Issue stablecoins equivalent to 1 BTC
}