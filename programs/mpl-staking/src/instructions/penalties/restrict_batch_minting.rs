use super::Penalty;
use crate::clock_unix_timestamp;
use anchor_lang::prelude::*;
use mplx_staking_states::error::MplStakingError;

/// Restricts batch minting operation for the account until the specified timestamp.
pub fn restrict_batch_minting(ctx: Context<Penalty>, until_ts: u64) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    require_keys_eq!(
        registrar.realm_authority,
        ctx.accounts.realm_authority.key(),
        MplStakingError::InvalidRealmAuthority
    );

    require!(
        until_ts > clock_unix_timestamp(),
        MplStakingError::InvalidTimestampArguments
    );

    let mut voter = ctx.accounts.voter.load_mut()?;
    voter.batch_minting_restricted_until = until_ts;

    Ok(())
}
