use super::Penalty;
use anchor_lang::prelude::*;
use mplx_staking_states::error::MplStakingError;

/// Restricts claiming rewards from the specified mining account.
pub fn restrict_tokenflow(ctx: Context<Penalty>) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    let mut voter = ctx.accounts.voter.load_mut()?;
    voter.restrict_tokenflow()?;

    Ok(())
}
