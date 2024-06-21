use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use mplx_staking_states::error::VsrError;
use mplx_staking_states::state::LockupKind;
use mplx_staking_states::state::LockupPeriod;
use mplx_staking_states::state::Registrar;
use mplx_staking_states::state::Voter;

use crate::clock_unix_timestamp;
use crate::cpi_instructions::extend_deposit;

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
        constraint = deposit_token.owner == deposit_authority.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,
    /// The owner of the deposit and its reward's mining account
    pub deposit_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = deposit_token.mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    /// PDA(["mining", mining owner <aka voter_authority in our case>, reward_pool], reward_program)
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    pub rewards_program: UncheckedAccount<'info>,
}

impl<'info> RestakeDeposit<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer {
            from: self.deposit_token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.deposit_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

/// Prolongs the deposit
///
/// The deposit will be restaked with the same lockup period as it was previously in case it's not expired.
/// If the deposit has expired, it can be restaked with any LockupPeriod.
/// The deposit entry must have been initialized with create_deposit_entry.
///
/// `deposit_entry_index`: Index of the deposit entry.
pub fn restake_deposit(
    ctx: Context<RestakeDeposit>,
    deposit_entry_index: u8,
    new_lockup_period: LockupPeriod,
    registrar_bump: u8,
    realm_governing_mint_pubkey: Pubkey,
    realm_pubkey: Pubkey,
    additional_amount: u64,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;
    let mining_owner = voter.voter_authority;
    let d_entry = voter.active_deposit_mut(deposit_entry_index)?;
    require!(
        d_entry.lockup.period != LockupPeriod::None && d_entry.lockup.kind != LockupKind::None,
        VsrError::RestakeDepositIsNotAllowed
    );
    let start_ts = d_entry.lockup.start_ts;
    let curr_ts = clock_unix_timestamp();
    let amount = d_entry.amount_deposited_native;
    let old_lockup_period = if d_entry.lockup.expired(curr_ts) {
        LockupPeriod::Flex
    } else {
        d_entry.lockup.period
    };

    // different type of deposit is only allowed if
    // the current deposit has expired
    if old_lockup_period != LockupPeriod::Flex {
        require!(
            new_lockup_period == d_entry.lockup.period,
            VsrError::RestakeDepositIsNotAllowed
        );
    }

    // Get the exchange rate entry associated with this deposit.
    let mint_idx = registrar.voting_mint_config_index(ctx.accounts.deposit_token.mint)?;
    require_eq!(
        mint_idx,
        d_entry.voting_mint_config_idx as usize,
        VsrError::InvalidMint
    );

    let reward_pool = &ctx.accounts.reward_pool;
    let mining = &ctx.accounts.deposit_mining;
    let pool_deposit_authority = &ctx.accounts.registrar;
    let base_amount = d_entry.amount_deposited_native;

    if additional_amount > 0 {
        // Deposit tokens into the vault and increase the lockup amount too.
        token::transfer(ctx.accounts.transfer_ctx(), additional_amount)?;
        d_entry.amount_deposited_native = d_entry
            .amount_deposited_native
            .checked_add(additional_amount)
            .unwrap();
    }

    let signers_seeds = &[
        &realm_pubkey.key().to_bytes(),
        b"registrar".as_ref(),
        &realm_governing_mint_pubkey.key().to_bytes(),
        &[registrar_bump][..],
    ];

    extend_deposit(
        ctx.accounts.rewards_program.to_account_info(),
        reward_pool.to_account_info(),
        mining.to_account_info(),
        pool_deposit_authority.to_account_info(),
        old_lockup_period,
        new_lockup_period,
        start_ts,
        base_amount,
        additional_amount,
        &mining_owner,
        signers_seeds,
    )?;

    d_entry.lockup.start_ts = curr_ts;
    d_entry.lockup.end_ts = curr_ts
        .checked_add(new_lockup_period.to_secs())
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
