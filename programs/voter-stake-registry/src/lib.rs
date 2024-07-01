use anchor_lang::prelude::*;
use instructions::*;
use mplx_staking_states::state::lockup::{LockupKind, LockupPeriod};

pub mod cpi_instructions;
pub mod events;
mod governance;
mod instructions;
pub mod voter;

// The program address.
declare_id!("9XZ7Ku7FYGVk3veKba6BRKTFXoYJyh4b4ZHC6MfaTUE8");

/// # Introduction
///
/// The governance registry is an "addin" to the SPL governance program that
/// allows one to both vote with many different ypes of tokens for voting and to
/// scale voting power as a linear function of time locked--subject to some
/// maximum upper bound.
///
/// The flow for voting with this program is as follows:
///
/// - Create a SPL governance realm.
/// - Create a governance registry account.
/// - Add exchange rates for any tokens one wants to deposit. For example, if one wants to vote with
///   tokens A and B, where token B has twice the voting power of token A, then the exchange rate of
///   B would be 2 and the exchange rate of A would be 1.
/// - Create a voter account.
/// - Deposit tokens into this program, with an optional lockup period.
/// - Vote.
///
/// Upon voting with SPL governance, a client is expected to call
/// `decay_voting_power` to get an up to date measurement of a given `Voter`'s
/// voting power for the given slot. If this is not done, then the transaction
/// will fail (since the SPL governance program will require the measurement
/// to be active for the current slot).
///
/// # Interacting with SPL Governance
///
/// This program does not directly interact with SPL governance via CPI.
/// Instead, it simply writes a `VoterWeightRecord` account with a well defined
/// format, which is then used by SPL governance as the voting power measurement
/// for a given user.
///
/// # Max Vote Weight
///
/// Given that one can use multiple tokens to vote, the max vote weight needs
/// to be a function of the total supply of all tokens, converted into a common
/// currency. For example, if you have Token A and Token B, where 1 Token B =
/// 10 Token A, then the `max_vote_weight` should be `supply(A) + supply(B)*10`
/// where both are converted into common decimals. Then, when calculating the
/// weight of an individual voter, one can convert B into A via the given
/// exchange rate, which must be fixed.
///
/// Note that the above also implies that the `max_vote_weight` must fit into
/// a u64.
#[program]
pub mod voter_stake_registry {
    use super::*;

    pub fn create_registrar(
        ctx: Context<CreateRegistrar>,
        registrar_bump: u8,
        fill_authority: Pubkey,
        distribution_authority: Pubkey,
    ) -> Result<()> {
        instructions::create_registrar(ctx, registrar_bump, fill_authority, distribution_authority)
    }

    pub fn configure_voting_mint(
        ctx: Context<ConfigureVotingMint>,
        idx: u16,
        grant_authority: Option<Pubkey>,
    ) -> Result<()> {
        instructions::configure_voting_mint(
            ctx,
            idx,
            grant_authority,
        )
    }

    pub fn create_voter(
        ctx: Context<CreateVoter>,
        voter_bump: u8,
        voter_weight_record_bump: u8,
    ) -> Result<()> {
        instructions::create_voter(ctx, voter_bump, voter_weight_record_bump)
    }

    pub fn create_deposit_entry(
        ctx: Context<CreateDepositEntry>,
        deposit_entry_index: u8,
        kind: LockupKind,
        period: LockupPeriod,
        delegate: Pubkey,
    ) -> Result<()> {
        instructions::create_deposit_entry(ctx, deposit_entry_index, kind, period, delegate)
    }

    pub fn deposit(ctx: Context<Deposit>, deposit_entry_index: u8, amount: u64) -> Result<()> {
        instructions::deposit(ctx, deposit_entry_index, amount)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        deposit_entry_index: u8,
        amount: u64,
        registrar_bump: u8,
        realm_governing_mint_pubkey: Pubkey,
        realm_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::withdraw(
            ctx,
            deposit_entry_index,
            amount,
            registrar_bump,
            realm_governing_mint_pubkey,
            realm_pubkey,
        )
    }

    pub fn close_deposit_entry(
        ctx: Context<CloseDepositEntry>,
        deposit_entry_index: u8,
    ) -> Result<()> {
        instructions::close_deposit_entry(ctx, deposit_entry_index)
    }

    pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
        instructions::update_voter_weight_record(ctx)
    }

    pub fn unlock_tokens(ctx: Context<UnlockTokens>, deposit_entry_index: u8) -> Result<()> {
        instructions::unlock_tokens(ctx, deposit_entry_index)
    }

    pub fn close_voter<'info>(ctx: Context<'_, '_, '_, 'info, CloseVoter<'info>>) -> Result<()> {
        instructions::close_voter(ctx)
    }

    pub fn log_voter_info(
        ctx: Context<LogVoterInfo>,
        deposit_entry_begin: u8,
        deposit_entry_count: u8,
    ) -> Result<()> {
        instructions::log_voter_info(ctx, deposit_entry_begin, deposit_entry_count)
    }

    pub fn lock_tokens(
        ctx: Context<LockTokens>,
        source_deposit_entry_index: u8,
        target_deposit_entry_index: u8,
        amount: u64,
        realm_governing_mint_pubkey: Pubkey,
        realm_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::lock_tokens(
            ctx,
            source_deposit_entry_index,
            target_deposit_entry_index,
            amount,
            realm_governing_mint_pubkey,
            realm_pubkey,
        )
    }

    pub fn extend_stake(
        ctx: Context<ExtendStake>,
        deposit_entry_index: u8,
        new_lockup_period: LockupPeriod,
        registrar_bump: u8,
        realm_governing_mint_pubkey: Pubkey,
        realm_pubkey: Pubkey,
        additional_amount: u64,
    ) -> Result<()> {
        instructions::extend_stake(
            ctx,
            deposit_entry_index,
            new_lockup_period,
            registrar_bump,
            realm_governing_mint_pubkey,
            realm_pubkey,
            additional_amount,
        )
    }

    pub fn claim(
        ctx: Context<Claim>,
        registrar_bump: u8,
        realm_governing_mint_pubkey: Pubkey,
        realm_pubkey: Pubkey,
    ) -> Result<u64> {
        instructions::claim(
            ctx,
            registrar_bump,
            realm_governing_mint_pubkey,
            realm_pubkey,
        )
    }
}
