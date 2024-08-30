use super::Penalty;
use crate::cpi_instructions;
use anchor_lang::prelude::*;
use mplx_staking_states::{error::MplStakingError, registrar_seeds};

/// Restricts claiming rewards from the specified mining account.
pub fn allow_tokenflow(ctx: Context<Penalty>, mining_owner: Pubkey) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let signers_seeds = registrar_seeds!(&registrar);

    cpi_instructions::allow_tokenflow(
        ctx.accounts.rewards_program.to_account_info(),
        ctx.accounts.registrar.to_account_info(),
        ctx.accounts.reward_pool.to_account_info(),
        ctx.accounts.deposit_mining.to_account_info(),
        &mining_owner,
        signers_seeds,
    )?;

    Ok(())
}
