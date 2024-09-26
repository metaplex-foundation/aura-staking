use crate::cpi_instructions;
use anchor_lang::prelude::*;
use mplx_staking_states::{
    error::MplStakingError,
    registrar_seeds,
    state::{Registrar, Voter},
};

#[derive(Accounts)]
pub struct ChangeAuthorizedAgent<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address is just an extra precaution,
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

/// Changes the authorized agent for the voter.
pub fn change_authorized_agent(ctx: Context<ChangeAuthorizedAgent>, agent: Pubkey) -> Result<()> {
    let voter = &mut ctx.accounts.voter.load_mut()?;
    voter.authorized_agent = agent;
    let voter_authority = voter.voter_authority;

    Ok(())
}
