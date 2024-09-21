use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use arch_program::{CurveProxy, BabelVault, LiquidityGauge, EmissionReceiver, Token, ArchUtxo, ArchSdk, TransferResult};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CurveDepositToken {
    pub babel: Token,
    pub crv: Token,
    pub curve_proxy: CurveProxy,
    pub vault: BabelVault,
    pub gauge: Option<LiquidityGauge>,
    pub lp_token: Option<Token>,
    pub emission_id: Option<u64>,
    pub symbol: String,
    pub name: String,
    pub total_supply: u128,
    pub decimals: u8,
    pub balance_of: HashMap<String, u128>,
    pub allowance: HashMap<String, HashMap<String, u128>>,
    pub reward_integral: [u128; 2],
    pub reward_rate: [u128; 2],
    pub last_update: i64,
    pub period_finish: i64,
    pub reward_integral_for: HashMap<String, [u128; 2]>,
    pub stored_pending_reward: HashMap<String, [u128; 2]>,
}

impl CurveDepositToken {
    pub fn new(babel: Token, crv: Token, curve_proxy: CurveProxy, vault: BabelVault) -> Self {
        Self {
            babel,
            crv,
            curve_proxy,
            vault,
            gauge: None,
            lp_token: None,
            emission_id: None,
            symbol: String::new(),
            name: String::new(),
            total_supply: 0,
            decimals: 18,
            balance_of: HashMap::new(),
            allowance: HashMap::new(),
            reward_integral: [0; 2],
            reward_rate: [0; 2],
            last_update: Utc::now().timestamp(),
            period_finish: 0,
            reward_integral_for: HashMap::new(),
            stored_pending_reward: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, gauge: LiquidityGauge) -> Result<(), &'static str> {
        if self.gauge.is_some() {
            return Err("Already initialized");
        }

        let lp_token = gauge.lp_token();
        self.lp_token = Some(lp_token.clone());
        self.gauge = Some(gauge);

        lp_token.approve(self.gauge.as_ref().unwrap(), u128::MAX);
        let lp_token_symbol = lp_token.symbol();
        self.name = format!("Babel {} Curve Deposit", lp_token_symbol);
        self.symbol = format!("babel-{}", lp_token_symbol);
        self.period_finish = Utc::now().timestamp() - 1;

        Ok(())
    }

    pub fn notify_registered_id(&mut self, assigned_ids: Vec<u64>) -> Result<(), &'static str> {
        if self.emission_id.is_some() {
            return Err("Already registered");
        }
        if assigned_ids.len() != 1 {
            return Err("Incorrect ID count");
        }
        self.emission_id = Some(assigned_ids[0]);
        Ok(())
    }

    pub fn deposit(&mut self, receiver: String, amount: u128) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("Cannot deposit zero");
        }

        let lp_token = self.lp_token.as_ref().unwrap();
        lp_token.transfer_from(&receiver, &self.curve_proxy, amount)?;
        self.gauge.as_ref().unwrap().deposit(amount, &self.curve_proxy)?;

        let balance = self.balance_of.entry(receiver.clone()).or_insert(0);
        *balance += amount;
        self.total_supply += amount;

        self.update_integrals(&receiver, *balance, self.total_supply);
        self.fetch_rewards_if_needed()?;

        // Emit transfer event (similar to Solidity's event system)
        println!("Transfer event: {} deposited {} LP tokens.", receiver, amount);
        Ok(())
    }

    pub fn withdraw(&mut self, receiver: String, amount: u128) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("Cannot withdraw zero");
        }

        let balance = self.balance_of.get_mut(&receiver).ok_or("Insufficient balance")?;
        if *balance < amount {
            return Err("Insufficient balance");
        }

        let lp_token = self.lp_token.as_ref().unwrap();
        self.curve_proxy.withdraw_from_gauge(self.gauge.as_ref().unwrap(), lp_token, amount, &receiver)?;

        *balance -= amount;
        self.total_supply -= amount;

        self.update_integrals(&receiver, *balance, self.total_supply);
        self.fetch_rewards_if_needed()?;

        println!("Withdraw event: {} withdrew {} LP tokens.", receiver, amount);
        Ok(())
    }

    fn claim_reward(&mut self, claimant: String, receiver: String) -> Result<[u128; 2], &'static str> {
        self.update_integrals(&claimant, *self.balance_of.get(&claimant).unwrap_or(&0), self.total_supply);

        let pending_rewards = self.stored_pending_reward.remove(&claimant).unwrap_or([0; 2]);
        self.crv.transfer(&receiver, pending_rewards[1])?;

        self.vault.transfer_allocated_tokens(&claimant, &receiver, pending_rewards[0]);

        println!(
            "Reward claimed: {} received {} BABEL and {} CRV.",
            receiver, pending_rewards[0], pending_rewards[1]
        );

        Ok(pending_rewards)
    }

    pub fn claimable_reward(&self, account: &String) -> Result<(u128, u128), &'static str> {
        let updated = std::cmp::min(self.period_finish, Utc::now().timestamp());
        let duration = updated - self.last_update;

        let balance = *self.balance_of.get(account).unwrap_or(&0);
        let mut amounts = [0u128; 2];

        for i in 0..2 {
            let mut integral = self.reward_integral[i];
            if self.total_supply > 0 {
                integral += (duration as u128 * self.reward_rate[i] * 1e18) / self.total_supply;
            }
            let integral_for = *self.reward_integral_for.get(account).unwrap_or(&[0, 0])[i];
            amounts[i] = *self.stored_pending_reward.get(account).unwrap_or(&[0, 0])[i]
                + ((balance * (integral - integral_for)) / 1e18);
        }

        Ok((amounts[0], amounts[1]))
    }

    fn fetch_rewards_if_needed(&mut self) -> Result<(), &'static str> {
        if Utc::now().timestamp() / 1.week() >= self.period_finish / 1.week() {
            self.fetch_rewards();
        }
        Ok(())
    }

    fn fetch_rewards(&mut self) {
        let mut babel_amount = self.vault.allocate_new_emissions(self.emission_id.unwrap());

        let mut crv_amount = 0;
        if let Ok(minted) = self.curve_proxy.mint_crv(self.gauge.as_ref().unwrap()) {
            crv_amount = minted;
        }

        if Utc::now().timestamp() < self.period_finish {
            let remaining = self.period_finish - Utc::now().timestamp();
            babel_amount += remaining as u128 * self.reward_rate[0];
            crv_amount += remaining as u128 * self.reward_rate[1];
        }

        self.reward_rate[0] = babel_amount / BIMA_REWARD_DURATION;
        self.reward_rate[1] = crv_amount / BIMA_REWARD_DURATION;
        self.last_update = Utc::now().timestamp();
        self.period_finish = Utc::now().timestamp() + BIMA_REWARD_DURATION;
    }

    fn update_integrals(&mut self, account: &String, balance: u128, supply: u128) {
        let updated = std::cmp::min(self.period_finish, Utc::now().timestamp());
        let duration = updated - self.last_update;

        if duration > 0 {
            self.last_update = Utc::now().timestamp();
        }

        for i in 0..2 {
            let mut integral = self.reward_integral[i];
            if duration > 0 && supply > 0 {
                integral += (duration as u128 * self.reward_rate[i] * 1e18) / supply;
                self.reward_integral[i] = integral;
            }

            if account != "0" {
                let integral_for = *self.reward_integral_for.get(account).unwrap_or(&[0, 0])[i];
                if integral > integral_for {
                    let pending = (balance * (integral - integral_for)) / 1e18;
                    self.stored_pending_reward.entry(account.clone()).or_insert([0; 2])[i] += pending as u128;
                    self.reward_integral_for.entry(account.clone()).or_insert([0; 2])[i] = integral;
                }
            }
        }
    }
}