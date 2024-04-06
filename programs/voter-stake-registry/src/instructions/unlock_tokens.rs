use crate::error::*;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
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
}

pub fn unlock_tokens(ctx: Context<UnlockTokens>, deposit_entry_index: u8) -> Result<()> {
    let registrar = &ctx.accounts.registrar.load()?;
    let voter = &mut ctx.accounts.voter.load_mut()?;
    let curr_ts = registrar.clock_unix_timestamp();

    let deposit_entry = voter.active_deposit_mut(deposit_entry_index)?;

    // Check whether unlock request is allowed
    if deposit_entry.lockup.cooldown_ends_ts.is_none() {
        if curr_ts >= deposit_entry.lockup.end_ts {
            let cooldown_ends_ts = curr_ts
                .checked_add(COOLDOWN_SECS)
                .ok_or(VsrError::InvalidTimestampArguments)?;
            deposit_entry.lockup.cooldown_ends_ts = Some(cooldown_ends_ts);
            Ok(())
        } else {
            Err(VsrError::DepositStillLocked.into())
        }
    } else {
        Err(VsrError::UnlockAlreadyRequested.into())
    }
}
