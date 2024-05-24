use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mplx_staking_states::error::*;
use mplx_staking_states::state::*;
use solana_program::{
    instruction::Instruction,
    program::{invoke, invoke_signed},
    system_program, sysvar,
};

use crate::cpi_instructions::RewardsInstruction;

#[derive(Accounts)]
pub struct CreateDepositEntry<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = registrar,
        has_one = voter_authority)]
    pub voter: AccountLoader<'info, Voter>,

    #[account(
        init_if_needed,
        associated_token::authority = voter,
        associated_token::mint = deposit_mint,
        payer = payer
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    pub voter_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub deposit_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// Creates a new deposit entry.
///
/// Initializes a deposit entry with the requested settings.
/// Will error if the deposit entry is already in use.
///
/// - `deposit_entry_index`: deposit entry to use
/// - `kind`: Type of lockup to use.
/// - `start_ts`: Start timestamp in seconds, defaults to current clock.
///    The lockup will end after `start + LockupPeriod::to_ts + COOLDOWNS_SECS.
///
///    Note that tokens will already be locked before start_ts, it only defines
///    the vesting start time and the anchor for the periods computation.
///
/// - `period`: An enum that represents possible options for locking up
pub fn create_deposit_entry(
    ctx: Context<CreateDepositEntry>,
    deposit_entry_index: u8,
    kind: LockupKind,
    start_ts: Option<u64>,
    period: LockupPeriod,
) -> Result<()> {
    // Load accounts.
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;

    // Get the exchange rate entry associated with this deposit.
    let mint_idx = registrar.voting_mint_config_index(ctx.accounts.deposit_mint.key())?;

    // Get and set up the deposit entry.
    require_gt!(
        voter.deposits.len(),
        deposit_entry_index as usize,
        VsrError::OutOfBoundsDepositEntryIndex
    );
    let d_entry = &mut voter.deposits[deposit_entry_index as usize];
    require!(!d_entry.is_used, VsrError::UnusedDepositEntryIndex);

    let curr_ts = registrar.clock_unix_timestamp();
    let start_ts = start_ts.unwrap_or(curr_ts);

    *d_entry = DepositEntry::default();
    d_entry.is_used = true;
    d_entry.voting_mint_config_idx = mint_idx as u8;
    d_entry.amount_deposited_native = 0;
    d_entry.lockup = Lockup::new(kind, start_ts, period)?;

    Ok(())
}

/// Rewards initialize mining
#[allow(clippy::too_many_arguments)]
pub fn initialize_mining<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    user: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    rent: AccountInfo<'a>,
) -> Result<()> {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new(payer.key(), true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let ix =
        Instruction::new_with_borsh(*program_id, &RewardsInstruction::InitializeMining, accounts);

    invoke(
        &ix,
        &[reward_pool, mining, user, payer, system_program, rent],
    )?;

    Ok(())
}

/// Rewards deposit mining
#[allow(clippy::too_many_arguments)]
pub fn deposit_mining<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    user: AccountInfo<'a>,
    deposit_authority: AccountInfo<'a>,
    amount: u64,
    lockup_period: LockupPeriod,
    signers_seeds: &[&[&[u8]]],
    reward_mint: &Pubkey,
) -> Result<()> {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(reward_mint.key(), false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new_readonly(deposit_authority.key(), true),
    ];

    let ix = Instruction::new_with_borsh(
        *program_id,
        &RewardsInstruction::DepositMining {
            amount,
            lockup_period,
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
