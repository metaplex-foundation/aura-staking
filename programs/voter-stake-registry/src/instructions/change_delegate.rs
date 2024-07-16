use crate::{clock_unix_timestamp, cpi_instructions, Stake};
use anchor_lang::prelude::*;
use mplx_staking_states::{
    error::VsrError,
    state::{LockupKind, Registrar, Voter},
};
use solana_program::clock::SECONDS_PER_DAY;
pub const DELEGATE_UPDATE_DIFF_THRESHOLD: u64 = 5 * SECONDS_PER_DAY;

#[derive(Accounts)]
pub struct ChangeDelegate<'info> {
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

    /// CHECK: Mining Account that belongs to Rewards Program and some delegate
    /// The address of the mining account on the rewards program
    /// derived from PDA(["mining", delegate wallet addr, reward_pool], rewards_program)
    /// Seeds derivation will be checked on the rewards contract
    #[account(mut)]
    pub old_delegate_mining: UncheckedAccount<'info>,

    /// CHECK: Mining Account that belongs to Rewards Program and some delegate
    /// The address of the mining account on the rewards program
    /// derived from PDA(["mining", delegate wallet addr, reward_pool], rewards_program)
    /// Seeds derivation will be checked on the rewards contract
    #[account(mut)]
    pub new_delegate_mining: UncheckedAccount<'info>,

    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    /// PDA(["reward_pool", deposit_authority <aka registrar in our case>, fill_authority],
    /// reward_program)
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    /// PDA(["mining", mining owner <aka voter_authority in our case>, reward_pool], reward_program)
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    #[account(executable)]
    pub rewards_program: UncheckedAccount<'info>,
}

/// Changes delegate for the existing stake.
///
/// Rewards will be recalculated, and the new delegate will start receiving rewards.
/// The old delegate will stop receiving rewards.
/// It might be done once per five days.
pub fn change_delegate(ctx: Context<ChangeDelegate>, deposit_entry_index: u8) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;
    let curr_ts = clock_unix_timestamp();
    let target = voter.active_deposit_mut(deposit_entry_index)?;
    let delegate_last_update_diff = curr_ts
        .checked_sub(target.delegate_last_update)
        .ok_or(VsrError::ArithmeticOverflow)?;

    require!(
        delegate_last_update_diff > DELEGATE_UPDATE_DIFF_THRESHOLD,
        VsrError::DelegateUpdateIsTooSoon
    );

    require!(
        ctx.accounts.old_delegate_mining.key() != &target.new_delegate_mining,
        VsrError::SameDelegate
    );

    let reward_pool = ctx.accounts.reward_pool.to_account_info();
    let mining = ctx.accounts.deposit_mining.to_account_info();
    let deposit_authority = ctx.accounts.registrar.to_account_info();
    let (delegate_mining, _) = ctx.accounts.delegate_mining.to_account_info();
    let signers_seeds = &[
        &registrar.realm.key().to_bytes(),
        b"registrar".as_ref(),
        &registrar.realm_governing_token_mint.key().to_bytes(),
        &[registrar.bump][..],
    ];
    let owner = &ctx.accounts.voter_authority.key();

    cpi_instructions::deposit_mining(
        ctx.accounts.rewards_program.to_account_info(),
        reward_pool,
        mining,
        deposit_authority,
        delegate_mining,
        amount,
        target.lockup.period,
        owner,
        signers_seeds,
    )?;

    Ok(())
}
