use crate::{error::*, state::deposit_entry::DepositEntry};
use anchor_lang::prelude::*;

/// User account for minting voting rights.
#[account(zero_copy)]
pub struct Voter {
    /// The deposits that the voter has made.
    pub deposits: [DepositEntry; 32],
    /// Authorized agent. This pubkey is authorized by the staker/voter to perform permissioned
    /// actions that require stake. This is the same as the voter_authority initially, but may be
    /// changed by the voter_authority in order to not expose the voter_authority's private key.
    pub authorized_agent: Pubkey,
    /// The voter_authority is the account that has the right to vote with the voter's stake. This
    /// is the account that will sign the vote transactions as well as the account that will sign
    /// the withdrawal transactions.
    pub voter_authority: Pubkey,
    /// The pubkey of the registrar that the voter is registered with.
    pub registrar: Pubkey,
    /// The total weighted stake that the voter was penalized for. This reduces the voter's
    /// effective stake.
    pub decreased_weighted_stake_by: u64,
    /// The batch minting is restricted until this timestamp.
    pub batch_minting_restricted_until: u64,
    /// The bump seed used to derive the voter_authority.
    pub voter_bump: u8,
    /// The bump seed used to derive the voter_weight_record.
    pub voter_weight_record_bump: u8,
    /// The bitmap of penalties that the voter has incurred.
    pub penalties: u8,
    /// Reserved for allignment and future use.
    pub _reserved1: [u8; 13],
}
const_assert!(std::mem::size_of::<Voter>() == 144 * 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1 + 13);
const_assert!(std::mem::size_of::<Voter>() % 8 == 0);

impl Voter {
    pub const MIN_OWN_WEIGHTED_STAKE: u64 = 15_000_000;
    const IS_TOKENFLOW_RESTRICTED_MASK: u8 = 1 << 0;

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
    pub fn weight_baseline(&self) -> u64 {
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .fold(0, |acc, d| acc + d.amount_deposited_native)
    }

    /// The extra lockup vote weight that the user is guaranteed to have at `at_ts`, assuming
    /// they withdraw and unlock as much as possible starting from `curr_ts`.
    pub fn weight_locked_guaranteed(&self, curr_ts: i64, at_ts: i64) -> Result<u64> {
        require_gte!(at_ts, curr_ts, MplStakingError::InvalidTimestampArguments);
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
            MplStakingError::OutOfBoundsDepositEntryIndex
        );
        let d = &mut self.deposits[index];
        require!(d.is_used, MplStakingError::UnusedDepositEntryIndex);
        Ok(d)
    }

    pub fn active_deposit(&self, index: u8) -> Result<&DepositEntry> {
        let index = index as usize;
        require_gt!(
            self.deposits.len(),
            index,
            MplStakingError::OutOfBoundsDepositEntryIndex
        );
        let d = &self.deposits[index];
        require!(d.is_used, MplStakingError::UnusedDepositEntryIndex);
        Ok(d)
    }

    pub fn restrict_tokenflow(&mut self) -> Result<()> {
        if self.is_tokenflow_restricted() {
            Err(MplStakingError::TokenflowRestrictedAlready.into())
        } else {
            self.penalties |= Self::IS_TOKENFLOW_RESTRICTED_MASK;
            Ok(())
        }
    }

    pub fn allow_tokenflow(&mut self) -> Result<()> {
        if !self.is_tokenflow_restricted() {
            Err(MplStakingError::TokenflowRestrictedAlready.into())
        } else {
            self.penalties &= !(Self::IS_TOKENFLOW_RESTRICTED_MASK);
            Ok(())
        }
    }

    pub fn is_tokenflow_restricted(&self) -> bool {
        self.penalties & Self::IS_TOKENFLOW_RESTRICTED_MASK > 0
    }

    pub fn is_batch_minting_restricted(&self) -> bool {
        let curr_ts = Clock::get().unwrap().unix_timestamp as u64;

        self.batch_minting_restricted_until > curr_ts
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
