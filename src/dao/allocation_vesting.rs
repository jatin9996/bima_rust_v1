// Define errors that might occur in the contract
#[derive(Debug)]
enum ContractError {
    NothingToClaim,
    CannotLock,
    WrongMaxTotalPreclaimPct,
    PreclaimTooLarge,
    AllocationsMismatch,
    ZeroTotalAllocation,
    ZeroAllocation,
    ZeroNumberOfWeeks,
    DuplicateAllocation,
    InsufficientPoints,
    LockedAllocation,
    IllegalVestingStart,
    VestingAlreadyStarted,
    IncompatibleVestingPeriod,
}

// Structs to mirror Solidity's structs
#[derive(Debug, Clone)]
struct AllocationSplit {
    recipient: String, // Using String to represent addresses
    points: u32,
    number_of_weeks: u8,
}

#[derive(Debug, Clone)]
struct AllocationState {
    points: u32,
    number_of_weeks: u8,
    claimed: u128,
    preclaimed: u96,
}

// Main contract struct
struct AllocationVesting {
    allocations: HashMap<String, AllocationState>,
    max_total_preclaim_pct: u32,
    total_allocation: u128,
    vesting_start: Option<u64>, // Using Option to represent uninitialized state
}

impl AllocationVesting {
    // Constructor to initialize the contract
    fn new(total_allocation: u128, max_total_preclaim_pct: u32) -> Result<Self, ContractError> {
        if total_allocation == 0 {
            return Err(ContractError::ZeroTotalAllocation);
        }
        if max_total_preclaim_pct > 20 {
            return Err(ContractError::WrongMaxTotalPreclaimPct);
        }

        Ok(Self {
            allocations: HashMap::new(),
            max_total_preclaim_pct,
            total_allocation,
            vesting_start: None,
        })
    }

    // Method to set allocations and start vesting
    fn set_allocations(&mut self, allocation_splits: Vec<AllocationSplit>, vesting_start: u64) -> Result<(), ContractError> {
        if self.vesting_start.is_some() {
            return Err(ContractError::VestingAlreadyStarted);
        }
        // Additional logic to check and set allocations
        Ok(())
    }

    // Additional methods to mirror Solidity functions
}

fn main() {
    // Example usage
    let mut contract = AllocationVesting::new(1000, 10).unwrap();
    let allocation_splits = vec![AllocationSplit {
        recipient: "0x123...".to_string(),
        points: 100,
        number_of_weeks: 52,
    }];
    contract.set_allocations(allocation_splits, 1234567890).unwrap();
}
