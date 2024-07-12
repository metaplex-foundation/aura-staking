use crate::clock_unix_timestamp;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mplx_staking_states::{
    error::VsrError,
    state::{DepositEntry, Lockup, LockupKind, LockupPeriod, Registrar, Voter},
};

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
}

/// Creates a new deposit entry.
///
/// Initializes a deposit entry with the requested settings.
/// Will error if the deposit entry is already in use.
///
/// - `deposit_entry_index`: deposit entry to use
/// - `kind`: Type of lockup to use.
/// - `start_ts`: Start timestamp in seconds, defaults to current clock. The lockup will end after
///   `start + LockupPeriod::to_ts + COOLDOWNS_SECS.
///
///    Note that tokens will already be locked before start_ts, it only defines
///    the vesting start time and the anchor for the periods computation.
///
/// - `period`: An enum that represents possible options for locking up
pub fn create_deposit_entry(
    ctx: Context<CreateDepositEntry>,
    deposit_entry_index: u8,
    kind: LockupKind,
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

    let start_ts = clock_unix_timestamp();
    *d_entry = DepositEntry::default();
    d_entry.is_used = true;
    d_entry.voting_mint_config_idx = mint_idx as u8;
    d_entry.amount_deposited_native = 0;
    d_entry.delegate_mining = *ctx.accounts.voter_authority.key;
    d_entry.lockup = Lockup::new(kind, start_ts, period)?;

    Ok(())
}
