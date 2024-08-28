pub use change_delegate::*;
pub use claim::*;
pub use close_deposit_entry::*;
pub use close_voter::*;
pub use configure_voting_mint::*;
pub use create_deposit_entry::*;
pub use create_registrar::*;
pub use create_voter::*;
pub use deposit::*;
pub use extend_stake::*;
pub use log_voter_info::*;
use solana_program::{clock::Clock, sysvar::Sysvar};
pub use stake::*;
pub use unlock_tokens::*;
pub use update_voter_weight_record::*;
pub use withdraw::*;

mod change_delegate;
mod claim;
mod close_deposit_entry;
mod close_voter;
mod configure_voting_mint;
mod create_deposit_entry;
mod create_registrar;
mod create_voter;
mod deposit;
mod extend_stake;
mod log_voter_info;
mod stake;
mod unlock_tokens;
mod update_voter_weight_record;
mod withdraw;

pub fn clock_unix_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
