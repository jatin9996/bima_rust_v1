// src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

pub use ink_lang as ink;

#[ink::contract]
mod debt_token {
    use ink_storage::collections::HashMap as StorageMap;
    use ink_storage::traits::SpreadAllocate;

    const FLASH_LOAN_FEE: u128 = 9; // 1 = 0.0001%

    #[ink(event)]
    pub struct Minted {
        #[ink(topic)]
        account: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Burned {
        #[ink(topic)]
        account: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Transferred {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct FlashLoaned {
        #[ink(topic)]
        receiver: AccountId,
        amount: Balance,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DebtToken {
        name: String,
        symbol: String,
        total_supply: Balance,
        balances: StorageMap<AccountId, Balance>,
        allowances: StorageMap<(AccountId, AccountId), Balance>,
        trove_managers: StorageMap<AccountId, bool>,
        _cached_domain_separator: [u8; 32],
        _cached_chain_id: u64,
        _hashed_name: [u8; 32],
        _hashed_version: [u8; 32],
        // Other necessary fields...
    }

    impl DebtToken {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let instance = Self {
                name,
                symbol,
                total_supply: 0,
                balances: StorageMap::new(),
                allowances: StorageMap::new(),
                trove_managers: StorageMap::new(),
                _cached_domain_separator: [0u8; 32], // Initialize appropriately
                _cached_chain_id: 0, // Initialize appropriately
                _hashed_name: [0u8; 32], // Initialize appropriately
                _hashed_version: [0u8; 32], // Initialize appropriately
            };
            instance
        }

        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) {
            // Mint logic
            let balance = self.balances.get(&account).unwrap_or(0);
            self.balances.insert(account, balance + amount);
            self.total_supply += amount;

            // Emit Minted event
            self.env().emit_event(Minted { account, amount });
        }

        #[ink(message)]
        pub fn burn(&mut self, account: AccountId, amount: Balance) {
            // Burn logic
            let balance = self.balances.get(&account).unwrap_or(0);
            assert!(balance >= amount, "Insufficient balance");
            self.balances.insert(account, balance - amount);
            self.total_supply -= amount;

            // Emit Burned event
            self.env().emit_event(Burned { account, amount });
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: Balance) {
            // Approve logic
            let caller = self.env().caller();
            self.allowances.insert((caller, spender), amount);
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) {
            // Transfer logic
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).unwrap_or(0);
            assert!(balance >= amount, "Insufficient balance");
            self.balances.insert(caller, balance - amount);
            let recipient_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.insert(to, recipient_balance + amount);

            // Emit Transferred event
            self.env().emit_event(Transferred { from: caller, to, amount });
        }

        #[ink(message)]
        pub fn flash_loan(
            &mut self,
            receiver: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), &'static str> {
            // Ensure the loan amount is valid
            assert!(amount <= self.max_flash_loan(), "Amount exceeds max flash loan");

            // Mint the tokens to the receiver
            self.mint(receiver, amount);

            // Call the receiver's onFlashLoan function
            let result = ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(ink_env::Call::from(receiver))
                .exec_input(ink_env::call::ExecutionInput::new(ink_env::call::Selector::new("onFlashLoan"))
                    .push_arg(self.env().account_id())
                    .push_arg(amount)
                    .push_arg(data))
                .returns::<()>()
                .fire();

            // Check the result of the call
            match result {
                Ok(_) => {
                    // Burn the tokens after the loan is repaid
                    self.burn(receiver, amount);
                    // Emit FlashLoaned event
                    self.env().emit_event(FlashLoaned { receiver, amount });
                    Ok(())
                },
                Err(_) => Err("Flash loan failed"),
            }
        }

        #[ink(message)]
        pub fn max_flash_loan(&self) -> Balance {
            self.total_supply // or any other logic to determine max loan
        }

        #[ink(message)]
        pub fn flash_fee(&self, amount: Balance) -> Balance {
            (amount * FLASH_LOAN_FEE) / 10000 // Assuming FLASH_LOAN_FEE is defined
        }

        #[ink(message)]
        pub fn domain_separator(&self) -> [u8; 32] {
            if self.env().chain_id() == self._cached_chain_id {
                self._cached_domain_separator
            } else {
                self._build_domain_separator()
            }
        }

        fn _build_domain_separator(&self) -> [u8; 32] {
            // EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)
            let type_hash = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
            let name_hash = keccak256(&self.name.as_bytes());
            let version_hash = keccak256(&self.symbol.as_bytes());
            let chain_id = self.env().chain_id();
            let contract_address = self.env().account_id();

            // Combine and hash the values to create the domain separator
            let domain_separator = keccak256(&[
                &type_hash,
                &name_hash,
                &version_hash,
                &chain_id.to_le_bytes(),
                &contract_address.encode(),
            ].concat());

            domain_separator
        }
    }
}