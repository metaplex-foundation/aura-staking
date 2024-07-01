pub use claim::*;
pub use close_deposit_entry::*;
pub use close_voter::*;
pub use configure_voting_mint::*;
pub use create_deposit_entry::*;
pub use create_registrar::*;
pub use create_voter::*;
pub use deposit::*;
pub use extend_stake::*;
pub use stake::*;
pub use log_voter_info::*;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
pub use unlock_tokens::*;
pub use update_voter_weight_record::*;
pub use withdraw::*;

mod claim;
mod close_deposit_entry;
mod close_voter;
mod configure_voting_mint;
mod create_deposit_entry;
mod create_registrar;
mod create_voter;
mod deposit;
mod extend_stake;
mod stake;
mod log_voter_info;
mod unlock_tokens;
mod update_voter_weight_record;
mod withdraw;

pub fn clock_unix_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
