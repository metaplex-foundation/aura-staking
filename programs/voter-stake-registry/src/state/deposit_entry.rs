use crate::error::*;
use crate::state::lockup::{Lockup, LockupKind};
use crate::state::voting_mint_config::VotingMintConfig;
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

    /// Amount in locked when the lockup began, in native currency.
    ///
    /// Note that this is not adjusted for withdraws. It is possible for this
    /// value to be bigger than amount_deposited_native after some vesting
    /// and withdrawals.
    ///
    /// This value is needed to compute the amount that vests each peroid,
    /// which should not change due to withdraws.
    pub amount_initially_locked_native: u64,

    // True if the deposit entry is being used.
    pub is_used: bool,

    /// If the clawback authority is allowed to extract locked tokens.
    pub allow_clawback: bool,

    // Points to the VotingMintConfig this deposit uses.
    pub voting_mint_config_idx: u8,

    pub reserved: [u8; 29],
}
const_assert!(std::mem::size_of::<DepositEntry>() == 32 + 2 * 8 + 3 + 29);
const_assert!(std::mem::size_of::<DepositEntry>() % 8 == 0);

impl DepositEntry {
    /// # Voting Power Caclulation
    /// ### Constant Lockup
    /// Voting Power will be always equals to 1*(locked + staked)
    /// since we don't provide any other methods besides constant locking
    pub fn voting_power(
        &self,
        _voting_mint_config: &VotingMintConfig,
        _curr_ts: i64,
    ) -> Result<u64> {
        self.amount_deposited_native
            .checked_add(self.amount_initially_locked_native)
            .ok_or_else(|| error!(VsrError::VoterWeightOverflow))
    }

    /// Vote power contribution from locked funds only.
    pub fn voting_power_locked(
        &self,
        curr_ts: i64,
        _max_locked_vote_weight: u64,
        _lockup_saturation_secs: u64,
    ) -> Result<u64> {
        if self.lockup.expired(curr_ts) {
            return Ok(0);
        }
        match self.lockup.kind {
            LockupKind::None => Ok(0),
            LockupKind::Constant => Ok(self.amount_initially_locked_native),
        }
    }

    /// Vote power contribution from locked funds only at `at_ts`, assuming the user does everything
    /// they can to unlock as quickly as possible at `curr_ts`.
    ///
    /// Currently that means that Constant will be unlocked immidiatelly.
    pub fn voting_power_locked_guaranteed(
        &self,
        _curr_ts: i64,
        _at_ts: i64,
        _max_locked_vote_weight: u64,
        _lockup_saturation_secs: u64,
    ) -> Result<u64> {
        Ok(0)
    }

    /// Returns native tokens still locked.
    #[inline(always)]
    pub fn amount_locked(&self, _curr_ts: i64) -> u64 {
        self.amount_initially_locked_native
    }

    /// Returns native tokens that are unlocked given current vesting
    /// and previous withdraws.
    #[inline(always)]
    pub fn amount_unlocked(&self, curr_ts: i64) -> u64 {
        self.amount_deposited_native
            .checked_sub(self.amount_locked(curr_ts))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LockupKind::Constant;

    #[test]
    pub fn far_future_lockup_start_test() -> Result<()> {
        // Check that voting power stays correct even if the lockup is very far in the
        // future, or at least more than lockup_saturation_secs in the future.
        let day: i64 = 86_400;
        let saturation: i64 = 5 * day;
        let lockup_start = 10_000_000_000; // arbitrary point
        let deposit = DepositEntry {
            amount_deposited_native: 10_000,
            amount_initially_locked_native: 10_000,
            lockup: Lockup {
                start_ts: lockup_start,
                end_ts: lockup_start + 2 * day,
                kind: Constant,
                reserved: [0; 15],
            },
            is_used: true,
            allow_clawback: false,
            voting_mint_config_idx: 0,
            reserved: [0; 29],
        };
        let voting_mint_config = VotingMintConfig {
            mint: Pubkey::default(),
            grant_authority: Pubkey::default(),
            baseline_vote_weight_scaled_factor: 1_000_000_000, // 1x
            max_extra_lockup_vote_weight_scaled_factor: 1_000_000_000, // 1x
            lockup_saturation_secs: saturation as u64,
            digit_shift: 0,
            reserved1: [0; 7],
            reserved2: [0; 7],
        };

        let baseline_vote_weight =
            voting_mint_config.baseline_vote_weight(deposit.amount_deposited_native)?;
        assert_eq!(baseline_vote_weight, 10_000);
        let max_locked_vote_weight = voting_mint_config
            .max_extra_lockup_vote_weight(deposit.amount_initially_locked_native)?;
        assert_eq!(max_locked_vote_weight, 10_000);

        // The timestamp 100_000 is very far before the lockup_start timestamp
        let withdrawable = deposit.amount_unlocked(100_000);
        assert_eq!(withdrawable, 0);
        let voting_power = deposit.voting_power(&voting_mint_config, 100_000).unwrap();
        assert_eq!(voting_power, 20_000);

        let voting_power = deposit
            .voting_power(&voting_mint_config, lockup_start - saturation)
            .unwrap();
        assert_eq!(voting_power, 20_000);

        let voting_power = deposit
            .voting_power(&voting_mint_config, lockup_start - saturation + day)
            .unwrap();
        assert_eq!(voting_power, 20_000);

        let voting_power = deposit
            .voting_power(&voting_mint_config, lockup_start - saturation + day + 1)
            .unwrap();
        assert_eq!(voting_power, 20_000);

        let voting_power = deposit
            .voting_power(&voting_mint_config, lockup_start - saturation + 2 * day)
            .unwrap();
        assert_eq!(voting_power, 20_000); // the second cliff has only 4/5th of lockup period left

        let voting_power = deposit
            .voting_power(&voting_mint_config, lockup_start - saturation + 2 * day + 1)
            .unwrap();
        assert_eq!(voting_power, 20_000);

        Ok(())
    }

    #[test]
    pub fn guaranteed_lockup_test() -> Result<()> {
        // Check that constant lockups are handled correctly.
        let day: i64 = 86_400;
        let saturation = (10 * day) as u64;
        let start = 10_000_000_000; // arbitrary point
        let deposit = DepositEntry {
            amount_deposited_native: 10_000,
            amount_initially_locked_native: 10_000,
            lockup: Lockup {
                start_ts: start,
                end_ts: start + 5 * day,
                kind: Constant,
                reserved: [0; 15],
            },
            is_used: true,
            allow_clawback: false,
            voting_mint_config_idx: 0,
            reserved: [0; 29],
        };

        let v = |curr_offset, at_offset| {
            deposit
                .voting_power_locked_guaranteed(
                    start + curr_offset,
                    start + at_offset,
                    100,
                    saturation,
                )
                .unwrap()
        };

        assert_eq!(v(0, 0), 0);
        assert_eq!(v(-day, 0), 0);
        assert_eq!(v(-100 * day, 0), 0);
        assert_eq!(v(-100 * day, -98 * day), 0);
        assert_eq!(v(0, day), 0);
        assert_eq!(v(0, 5 * day), 0);
        assert_eq!(v(0, 50 * day), 0);
        assert_eq!(v(day, day), 0);
        assert_eq!(v(day, 2 * day,), 0);
        assert_eq!(v(day, 20 * day), 0);
        assert_eq!(v(50 * day, 50 * day), 0);
        assert_eq!(v(50 * day, 51 * day), 0);
        assert_eq!(v(50 * day, 80 * day), 0);

        Ok(())
    }
}
