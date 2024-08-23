use std::collections::HashMap;
use crate::interfaces::babel_token::BabelToken;
use crate::interfaces::emission_schedule::EmissionSchedule;
use crate::interfaces::token_locker::ITokenLocker;
use crate::interfaces::boost_delegate::BoostDelegate;
use crate::interfaces::boost_calculator::BoostCalculator;
use crate::interfaces::incentive_voting::IIncentiveVoting;
use crate::dependecies::babel_ownable::BabelOwnable;
use crate::dependecies::system_start::SystemStart;

#[derive(Debug, Clone)]
struct Vault {
    babel_token: Box<dyn BabelToken>,
    emission_schedule: Box<dyn EmissionSchedule>,
    token_locker: Box<dyn ITokenLocker>,
    boost_calculator: Box<dyn BoostCalculator>,
    incentive_voting: Box<dyn IIncentiveVoting>,
    babel_ownable: BabelOwnable,
    system_start: SystemStart,
    unallocated_total: u128,
    weekly_emissions: HashMap<u64, u128>,
    allocated: HashMap<String, u128>,
}

impl Vault {
    fn new(
        babel_token: Box<dyn BabelToken>,
        emission_schedule: Box<dyn EmissionSchedule>,
        token_locker: Box<dyn ITokenLocker>,
        boost_calculator: Box<dyn BoostCalculator>,
        incentive_voting: Box<dyn IIncentiveVoting>,
        babel_ownable: BabelOwnable,
        system_start: SystemStart,
    ) -> Self {
        Vault {
            babel_token,
            emission_schedule,
            token_locker,
            boost_calculator,
            incentive_voting,
            babel_ownable,
            system_start,
            unallocated_total: 0, // This will be set later
            weekly_emissions: HashMap::new(),
            allocated: HashMap::new(),
        }
    }

    fn set_weekly_emission(&mut self, week: u64, amount: u128) {
        let (total_emissions, lock_amount) = self.emission_schedule.get_total_weekly_emissions(week, self.unallocated_total);
        self.weekly_emissions.insert(week, total_emissions);
        self.unallocated_total -= total_emissions;
        // Example of using the token locker
        self.token_locker.lock("vault", lock_amount, 52); // Lock for 1 year
    }

    fn transfer_tokens(&mut self, receiver: &str, amount: u128) {
        self.babel_token.transfer("vault", receiver, amount);
        self.unallocated_total -= amount;
    }

    fn increase_unallocated_supply(&mut self, amount: u128) {
        self.unallocated_total += amount;
        self.babel_token.increase_allowance("vault", amount);
    }
}

fn main() {
    // Example instantiation, assuming you have implementations for these traits
    let babel_token = Box::new(YourBabelTokenImplementation::new());
    let emission_schedule = Box::new(YourEmissionScheduleImplementation::new());
    let token_locker = Box::new(YourTokenLockerImplementation::new());
    let boost_calculator = Box::new(YourBoostCalculatorImplementation::new());
    let incentive_voting = Box::new(YourIncentiveVotingImplementation::new());
    let babel_ownable = BabelOwnable::new("owner_account_id");
    let system_start = SystemStart::new("babel_core_account_id");

    let mut vault = Vault::new(
        babel_token,
        emission_schedule,
        token_locker,
        boost_calculator,
        incentive_voting,
        babel_ownable,
        system_start,
    );

    vault.set_weekly_emission(1, 10000);
    vault.transfer_tokens("user1", 5000);
    vault.increase_unallocated_supply(2000);

    println!("{:?}", vault);
}