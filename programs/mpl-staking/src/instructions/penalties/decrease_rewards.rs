use crate::cpi_instructions;
use anchor_lang::prelude::*;
use mplx_staking_states::{
    error::MplStakingError,
    registrar_seeds,
    state::{Registrar, Voter},
};

#[derive(Accounts)]
pub struct DecreaseRewards<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    pub realm_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = registrar,
        has_one = voter_authority,
    )]
    pub voter: AccountLoader<'info, Voter>,
    /// CHECK: might be an arbitrary account
    pub voter_authority: UncheckedAccount<'info>,

    /// CHECK:
    /// Ownership of the account will be checked in the rewards contract
    /// It's the core account for the rewards contract, which will
    /// keep track of all rewards and staking logic.
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

/// Reduces the weighted stake of the mining account wich leads to a decrease in rewards.
///
/// - `decreased_weighted_stake_number`: weighted number to decrease by.
/// - `mining_owner`: The owner of the mining account.
pub fn decrease_rewards(
    ctx: Context<DecreaseRewards>,
    decreased_weighted_stake_number: u64,
) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let mut voter = ctx.accounts.voter.load_mut()?;
    voter.decreased_weighted_stake_by = voter
        .decreased_weighted_stake_by
        .checked_add(decreased_weighted_stake_number)
        .ok_or(MplStakingError::ArithmeticOverflow)?;

    let signers_seeds = registrar_seeds!(&registrar);
    let mining_owner = ctx.accounts.voter_authority.to_account_info().key;

    cpi_instructions::decrease_rewards(
        ctx.accounts.rewards_program.to_account_info(),
        ctx.accounts.registrar.to_account_info(),
        ctx.accounts.reward_pool.to_account_info(),
        ctx.accounts.deposit_mining.to_account_info(),
        decreased_weighted_stake_number,
        &mining_owner,
        signers_seeds,
    )?;

    Ok(())
}
