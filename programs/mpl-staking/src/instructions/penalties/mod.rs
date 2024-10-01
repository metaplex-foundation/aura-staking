pub use allow_tokenflow::*;
use anchor_lang::{
    prelude::{AccountLoader, Signer, ToAccountInfo},
    Accounts,
};
pub use decrease_rewards::*;
pub use restrict_batch_minting::*;
pub use restrict_tokenflow::*;
pub use slash::*;

mod allow_tokenflow;
mod decrease_rewards;
mod restrict_batch_minting;
mod restrict_tokenflow;
mod slash;

use mplx_staking_states::state::{Registrar, Voter};

#[derive(Accounts)]
pub struct Penalty<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    pub realm_authority: Signer<'info>,

    #[account(mut)]
    pub voter: AccountLoader<'info, Voter>,
}
