use crate::error::*;
use crate::state::deposit_entry::DepositEntry;
use crate::state::registrar::Registrar;
use anchor_lang::prelude::*;

/// User account for minting voting rights.
#[account(zero_copy)]
pub struct Voter {
    pub deposits: [DepositEntry; 32],
    pub voter_authority: Pubkey,
    pub registrar: Pubkey,
    pub voter_bump: u8,
    pub voter_weight_record_bump: u8,
    pub _reserved1: [u8; 14],
}
const_assert!(std::mem::size_of::<Voter>() == 80 * 32 + 32 + 32 + 1 + 1 + 14);
const_assert!(std::mem::size_of::<Voter>() % 8 == 0);

impl Voter {
    /// The full vote weight available to the voter
    pub fn weight(&self) -> Result<u64> {
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .try_fold(0_u64, |sum, d| {
                d.voting_power().map(|vp| sum.checked_add(vp).unwrap())
            })
    }

    /// The vote weight available to the voter when ignoring any lockup effects
    pub fn weight_baseline(&self, registrar: &Registrar) -> Result<u64> {
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .try_fold(0_u64, |sum, d| {
                registrar.voting_mints[d.voting_mint_config_idx as usize]
                    .baseline_vote_weight(d.amount_deposited_native)
                    .map(|vp| sum.checked_add(vp).unwrap())
            })
    }

    /// The extra lockup vote weight that the user is guaranteed to have at `at_ts`, assuming
    /// they withdraw and unlock as much as possible starting from `curr_ts`.
    pub fn weight_locked_guaranteed(&self, curr_ts: i64, at_ts: i64) -> Result<u64> {
        require_gte!(at_ts, curr_ts, VsrError::InvalidTimestampArguments);
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .try_fold(0_u64, |sum, _d| Ok(sum))
    }

    pub fn active_deposit_mut(&mut self, index: u8) -> Result<&mut DepositEntry> {
        let index = index as usize;
        require_gt!(
            self.deposits.len(),
            index,
            VsrError::OutOfBoundsDepositEntryIndex
        );
        let d = &mut self.deposits[index];
        require!(d.is_used, VsrError::UnusedDepositEntryIndex);
        Ok(d)
    }

    pub fn active_deposit(&self, index: u8) -> Result<&DepositEntry> {
        let index = index as usize;
        require_gt!(
            self.deposits.len(),
            index,
            VsrError::OutOfBoundsDepositEntryIndex
        );
        let d = &self.deposits[index];
        require!(d.is_used, VsrError::UnusedDepositEntryIndex);
        Ok(d)
    }
}

#[macro_export]
macro_rules! voter_seeds {
    ( $voter:expr ) => {
        &[
            $voter.registrar.as_ref(),
            b"voter".as_ref(),
            $voter.voter_authority.as_ref(),
            &[$voter.voter_bump],
        ]
    };
}

pub use voter_seeds;
