use crate::state::lockup::{Lockup, LockupKind};

use anchor_lang::prelude::*;

/// Bookkeeping for a single deposit for a given mint and lockup schedule.
#[zero_copy]
#[derive(Default)]
pub struct DepositEntry {
    // Locked state.
    pub lockup: Lockup,

    /// Amount in deposited, in native currency. Withdraws of vested tokens
    /// directly reduce this amount.
    ///
    /// This directly tracks the total amount added by the user. They may
    /// never withdraw more than this amount.
    pub amount_deposited_native: u64,
    // Points to the VotingMintConfig this deposit uses.
    pub voting_mint_config_idx: u8,
    // True if the deposit entry is being used.
    pub is_used: bool,
    pub _reserved1: [u8; 6],
}
const_assert!(std::mem::size_of::<DepositEntry>() == 32 + 8 + 1 + 1 + 6);
const_assert!(std::mem::size_of::<DepositEntry>() % 8 == 0);

impl DepositEntry {
    /// # Voting Power Caclulation
    /// ### Constant Lockup
    /// Voting Power will always be equal to 1*deposited
    /// since we don't provide any other methods besides constant locking
    pub fn voting_power(&self) -> Result<u64> {
        Ok(self.amount_deposited_native)
    }

    /// Returns native tokens still locked.
    #[inline(always)]
    pub fn amount_locked(&self, curr_ts: u64) -> u64 {
        let unlocked_tokens = if self.lockup.expired(curr_ts) {
            self.amount_deposited_native
        } else {
            0
        };
        self.amount_deposited_native
            .checked_sub(unlocked_tokens)
            .unwrap()
    }

    /// Returns native tokens that are unlocked given current vesting
    /// and previous withdraws.
    #[inline(always)]
    pub fn amount_unlocked(&self, curr_ts: u64) -> u64 {
        if let LockupKind::None = self.lockup.kind {
            return self.amount_deposited_native;
        }

        self.amount_deposited_native
            .checked_sub(self.amount_locked(curr_ts))
            .unwrap()
    }

    /// Returns the weighted stake for the given deposit at the specified timestamp.
    #[inline(always)]
    pub fn weighted_stake(&self, curr_ts: u64) -> u64 {
        if !self.is_used {
            return 0;
        }
        self.lockup.multiplier(curr_ts) * self.amount_deposited_native
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{LockupKind::Constant, LockupPeriod, VotingMintConfig};

    #[test]
    pub fn far_future_lockup_start_test() -> Result<()> {
        // Check that voting power stays correct even if the lockup is very far in the
        // future, or at least more than lockup_saturation_secs in the future.
        let day: i64 = 86_400;
        let saturation: i64 = 5 * day;
        let lockup_start = 10_000_000_000; // arbitrary point
        let period = LockupPeriod::Flex;
        let deposit = DepositEntry {
            amount_deposited_native: 20_000,
            lockup: Lockup {
                start_ts: lockup_start,
                end_ts: lockup_start + LockupPeriod::Flex.to_secs(), // start + cooldown + period
                kind: Constant,
                period,
                cooldown_requested: false,
                cooldown_ends_at: 0,
                _reserved1: [0; 5],
            },
            is_used: true,
            voting_mint_config_idx: 0,
            _reserved1: [0; 6],
        };
        let voting_mint_config = VotingMintConfig {
            mint: Pubkey::default(),
            grant_authority: Pubkey::default(),
            baseline_vote_weight_scaled_factor: 1_000_000_000, // 1x
            max_extra_lockup_vote_weight_scaled_factor: 1_000_000_000, // 1x
            lockup_saturation_secs: saturation as u64,
            digit_shift: 0,
            padding: [0; 7],
        };

        let baseline_vote_weight =
            voting_mint_config.baseline_vote_weight(deposit.amount_deposited_native)?;
        assert_eq!(baseline_vote_weight, 20_000);

        // The timestamp 100_000 is very far before the lockup_start timestamp
        let withdrawable = deposit.amount_unlocked(100_000);
        assert_eq!(withdrawable, 0);

        let voting_power = deposit.voting_power().unwrap();
        assert_eq!(voting_power, 20_000);

        Ok(())
    }

    #[test]
    fn test_weighted_stake_unused() {
        let deposit = DepositEntry {
            amount_deposited_native: 20_000,
            lockup: Lockup::default(),
            is_used: false,
            voting_mint_config_idx: 0,
            _reserved1: [0; 6],
        };
        assert_eq!(deposit.weighted_stake(0), 0);
    }

    #[test]
    fn test_weighted_stake_expired() {
        let amount = 20_000;
        let deposit = DepositEntry {
            amount_deposited_native: amount,
            lockup: Lockup {
                start_ts: 0,
                end_ts: 0,
                kind: Constant,
                period: LockupPeriod::Flex,
                cooldown_requested: false,
                cooldown_ends_at: 0,
                _reserved1: [0; 5],
            },
            is_used: true,
            voting_mint_config_idx: 0,
            _reserved1: [0; 6],
        };
        assert_eq!(deposit.weighted_stake(10), amount);
    }

    #[test]
    fn test_weighted_stake_under_cooldown() {
        let amount = 20_000;
        let deposit = DepositEntry {
            amount_deposited_native: amount,
            lockup: Lockup {
                start_ts: 0,
                end_ts: 100,
                kind: Constant,
                period: LockupPeriod::Flex,
                cooldown_requested: true,
                cooldown_ends_at: 200,
                _reserved1: [0; 5],
            },
            is_used: true,
            voting_mint_config_idx: 0,
            _reserved1: [0; 6],
        };
        assert_eq!(deposit.weighted_stake(150), 0);
    }

    #[test]
    fn test_weighted_stake() {
        let amount = 20_000;
        let deposit = DepositEntry {
            amount_deposited_native: amount,
            lockup: Lockup {
                start_ts: 0,
                end_ts: 100,
                kind: Constant,
                period: LockupPeriod::OneYear,
                cooldown_requested: false,
                cooldown_ends_at: 0,
                _reserved1: [0; 5],
            },
            is_used: true,
            voting_mint_config_idx: 0,
            _reserved1: [0; 6],
        };
        assert_eq!(
            deposit.weighted_stake(50),
            amount * LockupPeriod::OneYear.multiplier()
        );
    }
}
