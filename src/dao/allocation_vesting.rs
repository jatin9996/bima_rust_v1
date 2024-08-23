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

// Import necessary traits and structs
use crate::interfaces::token_locker::ITokenLocker;
use crate::dependecies::delegated_ops::DelegatedOps;
use crate::dependecies::babel_ownable::{BabelOwnable, IBabelCore};

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
    babel_ownable: BabelOwnable,
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
            babel_ownable: BabelOwnable::new(),
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

    // Implementing a new method to lock allocations using ITokenLocker
    fn lock_allocation(&self, account: String, amount: u256, weeks: u256) -> Result<(), String> {
        // Assuming there's a global TOKEN_LOCKER that implements ITokenLocker
        TOKEN_LOCKER.lock(account, amount, weeks)
    }
}

// Implementing the IBabelCore trait for AllocationVesting
impl IBabelCore for AllocationVesting {
    fn owner(&self) -> AccountId {
        // Assuming BabelOwnable is part of AllocationVesting
        self.babel_ownable.owner()
    }

    fn guardian(&self) -> AccountId {
        // Assuming BabelOwnable is part of AllocationVesting
        self.babel_ownable.guardian()
    }
}

// Additional modifications to integrate DelegatedOps
impl AllocationVesting {
    fn set_delegate_approval(&mut self, delegate: AccountId, is_approved: bool) {
        // Assuming there's a global DELEGATED_OPS that handles delegate approvals
        DELEGATED_OPS.set_delegate_approval(delegate, is_approved);
    }

    fn is_approved_delegate(&self, owner: AccountId, caller: AccountId) -> bool {
        // Assuming there's a global DELEGATED_OPS
        DELEGATED_OPS.is_approved_delegate(owner, caller)
    }
}

fn main() {
    
    let mut contract = AllocationVesting::new(1000, 10).unwrap();
    let allocation_splits = vec![AllocationSplit {
        recipient: "0x123...".to_string(),
        points: 100,
        number_of_weeks: 52,
    }];
    contract.set_allocations(allocation_splits, 1234567890).unwrap();
}