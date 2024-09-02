use super::Penalty;
use crate::cpi_instructions;
use anchor_lang::prelude::*;
use mplx_staking_states::error::MplStakingError;

/// Restricts claiming rewards from the specified mining account.
pub fn allow_tokenflow(ctx: Context<Penalty>, mining_owner: Pubkey) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let signers_seeds = &[
        &registrar.realm.key().to_bytes(),
        b"registrar".as_ref(),
        &registrar.realm_governing_token_mint.key().to_bytes(),
        &[registrar.bump][..],
    ];

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
