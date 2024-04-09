use crate::error::*;
use crate::state::deposit_entry::DepositEntry;
use crate::state::registrar::Registrar;
use anchor_lang::prelude::*;
use spl_governance::state::token_owner_record;

/// User account for minting voting rights.
#[account(zero_copy)]
pub struct Voter {
    pub voter_authority: Pubkey,
    pub registrar: Pubkey,
    pub deposits: [DepositEntry; 32],
    pub voter_bump: u8,
    pub voter_weight_record_bump: u8,
    pub reserved: [u8; 6],
}
const_assert!(std::mem::size_of::<Voter>() == 2 * 32 + 32 * 64 + 2 + 6);
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

    pub fn load_token_owner_record(
        &self,
        account_info: &AccountInfo,
        registrar: &Registrar,
    ) -> Result<token_owner_record::TokenOwnerRecordV2> {
        let record = token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint(
            &registrar.governance_program_id,
            account_info,
            &registrar.realm,
            &registrar.realm_governing_token_mint,
        )?;
        require_keys_eq!(
            record.governing_token_owner,
            self.voter_authority,
            VsrError::InvalidTokenOwnerRecord
        );
        Ok(record)
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
