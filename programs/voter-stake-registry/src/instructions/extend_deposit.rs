use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use mplx_staking_states::error::*;
use mplx_staking_states::state::*;

use crate::cpi_instructions::extend_deposit;
use crate::cpi_instructions::REWARD_CONTRACT_ID;

#[derive(Accounts)]
pub struct RestakeDeposit<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter.load()?.voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = registrar)]
    pub voter: AccountLoader<'info, Voter>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = deposit_token.mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = deposit_token.owner == deposit_authority.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,
    pub deposit_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    pub rewards_program: UncheckedAccount<'info>,
}

/// Prolongs the deposit
///
/// The deposit will be restaked with the same lockup period as it was previously.
///
/// The deposit entry must have been initialized with create_deposit_entry.
///
/// `deposit_entry_index`: Index of the deposit entry.
pub fn restake_deposit(
    ctx: Context<RestakeDeposit>,
    deposit_entry_index: u8,
    lockup_period: LockupPeriod,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;

    let d_entry = voter.active_deposit_mut(deposit_entry_index)?;

    // Get the exchange rate entry associated with this deposit.
    let mint_idx = registrar.voting_mint_config_index(ctx.accounts.deposit_token.mint)?;
    require_eq!(
        mint_idx,
        d_entry.voting_mint_config_idx as usize,
        VsrError::InvalidMint
    );

    let start_ts = d_entry.lockup.start_ts;
    let curr_ts = registrar.clock_unix_timestamp();
    let amount = d_entry.amount_deposited_native;

    if lockup_period != LockupPeriod::Flex {
        require!(
            lockup_period == d_entry.lockup.period,
            VsrError::RestakeDepositIsNotAllowed
        );
    }

    let reward_pool = &ctx.accounts.reward_pool;
    let mining = &ctx.accounts.deposit_mining;
    let deposit_authority = &ctx.accounts.deposit_authority;
    let reward_mint = &ctx.accounts.deposit_token.mint;
    let voter = &ctx.accounts.voter;

    let (_reward_pool_pubkey, pool_bump_seed) = Pubkey::find_program_address(
        &[&reward_pool.key().to_bytes(), &reward_mint.key().to_bytes()],
        &REWARD_CONTRACT_ID,
    );

    let signers_seeds = &[
        &reward_pool.key().to_bytes()[..32],
        &reward_mint.key().to_bytes()[..32],
        &[pool_bump_seed],
    ];

    extend_deposit(
        &REWARD_CONTRACT_ID,
        reward_pool.to_account_info(),
        mining.to_account_info(),
        reward_mint,
        voter.to_account_info(),
        deposit_authority.to_account_info(),
        amount,
        lockup_period,
        start_ts,
        &[signers_seeds],
    )?;

    d_entry.lockup.start_ts = curr_ts;
    d_entry.lockup.end_ts = curr_ts
        .checked_add(lockup_period.to_secs())
        .ok_or(VsrError::InvalidTimestampArguments)?;

    msg!(
        "Restaked deposit with amount {} at deposit index {} with lockup kind {:?} with lockup period {:?} and {} seconds left. It's used now: {:?}",
        amount,
        deposit_entry_index,
        d_entry.lockup.kind,
        d_entry.lockup.period,
        d_entry.lockup.seconds_left(curr_ts),
        d_entry.is_used,
    );

    Ok(())
}