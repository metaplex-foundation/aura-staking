use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use mplx_staking_states::error::*;
use mplx_staking_states::state::*;
use solana_program::{instruction::Instruction, program::invoke_signed};

use crate::cpi_instructions::RewardsInstruction;
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
    let d_enty_lockup_period = d_entry.lockup.period;
    let amount = d_entry.amount_deposited_native;

    if lockup_period != LockupPeriod::Flex {
        require!(
            lockup_period == d_entry.lockup.period,
            VsrError::RestakeDepositIsNotAllowed
        );
    }
    // TODO: call restake cpi

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

/// Restake deposit
#[allow(clippy::too_many_arguments)]
fn restake_deposit_cpi<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    reward_mint: &Pubkey,
    user: AccountInfo<'a>,
    deposit_authority: AccountInfo<'a>,
    amount: u64,
    lockup_period: LockupPeriod,
    deposit_start_ts: u64,
    signers_seeds: &[&[&[u8]]],
) -> Result<()> {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(*reward_mint, false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new_readonly(deposit_authority.key(), true),
    ];

    let ix = Instruction::new_with_borsh(
        *program_id,
        &RewardsInstruction::RestakeDeposit {
            lockup_period,
            amount,
            deposit_start_ts,
        },
        accounts,
    );

    invoke_signed(
        &ix,
        &[reward_pool, mining, user, deposit_authority],
        signers_seeds,
    )?;

    Ok(())
}
