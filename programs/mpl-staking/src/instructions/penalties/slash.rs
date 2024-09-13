use crate::{clock_unix_timestamp, cpi_instructions, voter::VoterWeightRecord};
use anchor_lang::prelude::*;
use mplx_staking_states::{
    error::MplStakingError,
    registrar_seeds,
    state::{Registrar, Voter},
};

#[derive(Accounts)]
pub struct Slashing<'info> {
    /// The voting registrar. There can only be a single registrar
    /// per governance realm and governing mint.
    pub registrar: AccountLoader<'info, Registrar>,

    #[account(mut)]
    pub voter: AccountLoader<'info, Voter>,

    pub realm_authority: Signer<'info>,

    /// Slashes must update the voter weight record, to prevent a stale
    /// record being used to vote after the slashing.
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter.load()?.voter_authority.key().as_ref()],
        bump = voter.load()?.voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.load()?.realm,
        constraint = voter_weight_record.governing_token_owner == voter.load()?.voter_authority,
        constraint = voter_weight_record.governing_token_mint == registrar.load()?.realm_governing_token_mint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// CHECK:
    /// Ownership of the account will be checked in the rewards contract
    /// It's the core account for the rewards contract, which will
    /// keep track of all rewards and staking logic.
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    /// PDA(["mining", mining owner <aka voter_authority in our case>, reward_pool],
    /// reward_program)
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    #[account(executable)]
    pub rewards_program: UncheckedAccount<'info>,
}

/// Slashes the specified stake in transfers money to the treasury.
///
/// `deposit_entry_index`: The deposit entry to slash.
/// `amount`: is in units of the native currency being slashed.
/// `mining_owner`: The owner of the mining account.
pub fn slash(
    ctx: Context<Slashing>,
    deposit_entry_index: u8,
    amount: u64,
    mining_owner: Pubkey,
) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let voter = &mut ctx.accounts.voter.load_mut()?;
    let deposit_entry = voter.active_deposit_mut(deposit_entry_index)?;

    // Bookkeeping for slashed funds.
    require_gte!(
        deposit_entry.amount_deposited_native,
        amount,
        MplStakingError::InternalProgramError
    );
    deposit_entry.slashing_penalty = deposit_entry
        .slashing_penalty
        .checked_add(amount)
        .ok_or(MplStakingError::ArithmeticOverflow)?;
    deposit_entry.amount_deposited_native = deposit_entry
        .amount_deposited_native
        .checked_sub(amount)
        .ok_or(MplStakingError::ArithmeticOverflow)?;
    // NB: accounts won't be closed automatically, even in case
    // of slashing. The user will have to withdraw the remaining
    // funds manually even if they're equal to zero.

    msg!(
        "Slashed amount {} at deposit index {}",
        amount,
        deposit_entry_index,
    );

    let slash_amount_multiplied_by_period = amount
        .checked_mul(deposit_entry.lockup.period.multiplier())
        .ok_or(MplStakingError::ArithmeticOverflow)?;
    let curr_ts = clock_unix_timestamp();
    let stake_expiration_date = if curr_ts > deposit_entry.lockup.end_ts {
        None
    } else {
        Some(deposit_entry.lockup.end_ts)
    };
    let signers_seeds = registrar_seeds!(&registrar);

    cpi_instructions::slash(
        ctx.accounts.rewards_program.to_account_info(),
        ctx.accounts.registrar.to_account_info(),
        ctx.accounts.reward_pool.to_account_info(),
        ctx.accounts.deposit_mining.to_account_info(),
        &mining_owner,
        amount,
        slash_amount_multiplied_by_period,
        stake_expiration_date,
        signers_seeds,
    )?;

    // Update the voter weight record
    let record = &mut ctx.accounts.voter_weight_record;
    record.voter_weight = voter.weight()?;
    record.voter_weight_expiry = Some(Clock::get()?.slot);

    Ok(())
}
