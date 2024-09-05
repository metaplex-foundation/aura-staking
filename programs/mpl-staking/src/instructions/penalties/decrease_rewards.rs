use super::Penalty;
use crate::cpi_instructions;
use anchor_lang::prelude::*;
use mplx_staking_states::{error::MplStakingError, registrar_seeds};

/// Reduces the weighted stake of the mining account wich leads to a decrease in rewards.
///
/// - `decreased_weighted_stake_number`: weighted number to decrease by.
/// - `mining_owner`: The owner of the mining account.
pub fn decrease_rewards(
    ctx: Context<Penalty>,
    decreased_weighted_stake_number: u64,
    mining_owner: Pubkey,
) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let signers_seeds = registrar_seeds!(&registrar);

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
