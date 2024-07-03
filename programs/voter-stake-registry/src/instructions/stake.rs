use crate::{clock_unix_timestamp, cpi_instructions};
use anchor_lang::prelude::*;
use mplx_staking_states::{
    error::VsrError,
    state::{LockupKind, Registrar, Voter},
};

#[derive(Accounts)]
pub struct Stake<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = voter_authority,
        has_one = registrar)]
    pub voter: AccountLoader<'info, Voter>,
    pub voter_authority: Signer<'info>,

    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    /// PDA(["reward_pool", deposit_authority <aka registrar in our case>, fill_authority],
    /// reward_program)
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

/// Transfers unlocked tokens from the source deposit entry to the target deposit entry.
///
/// Transfers token from one DepositEntry that is not LockupKind::None to another that is
/// LockupKind::Constant. In terms of business logic that means we want to deposit some tokens on
/// DAO, then we want to lock them up in order to receice rewards
pub fn stake(
    ctx: Context<Stake>,
    source_deposit_entry_index: u8,
    target_deposit_entry_index: u8,
    amount: u64,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;
    let curr_ts = clock_unix_timestamp();

    let source = voter.active_deposit_mut(source_deposit_entry_index)?;
    let source_mint_idx = source.voting_mint_config_idx;
    require!(
        source.lockup.kind == LockupKind::None,
        VsrError::LockingIsForbidded
    );

    // Reduce source amounts
    require_gte!(
        source.amount_unlocked(curr_ts),
        amount,
        VsrError::InsufficientUnlockedTokens
    );
    source.amount_deposited_native = source.amount_deposited_native.checked_sub(amount).unwrap();

    // Check target compatibility
    let target = voter.active_deposit_mut(target_deposit_entry_index)?;
    require_eq!(
        target.voting_mint_config_idx,
        source_mint_idx,
        VsrError::InvalidMint
    );

    // Checks that target doesn't have any stored tokens yet
    require!(
        target.amount_deposited_native == 0,
        VsrError::DepositEntryIsOld
    );
    // Add target amounts
    target.amount_deposited_native = target.amount_deposited_native.checked_add(amount).unwrap();

    let reward_pool = &ctx.accounts.reward_pool;
    let mining = &ctx.accounts.deposit_mining;
    let pool_deposit_authority = &ctx.accounts.registrar.to_account_info();
    let signers_seeds = &[
        &registrar.realm.key().to_bytes(),
        b"registrar".as_ref(),
        &registrar.realm_governing_token_mint.key().to_bytes(),
        &[registrar.bump][..],
    ];
    let owner = &ctx.accounts.voter_authority.key();

    cpi_instructions::deposit_mining(
        ctx.accounts.rewards_program.to_account_info(),
        reward_pool.to_account_info(),
        mining.to_account_info(),
        pool_deposit_authority.to_account_info(),
        amount,
        target.lockup.period,
        owner,
        signers_seeds,
    )?;

    Ok(())
}
