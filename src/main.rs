mod vault {
    pub mod stablecoin_vault;
    pub mod vault_operations;
}

use crate::vault::vault_operations::VaultOperations;

fn main() {
    let mut vault_ops = VaultOperations::new();

}