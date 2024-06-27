use solana_program::{clock::Clock, sysvar::Sysvar};
pub use {
    claim::*, close_deposit_entry::*, close_voter::*, configure_voting_mint::*,
    create_deposit_entry::*, create_registrar::*, create_voter::*, deposit::*, extend_deposit::*,
    lock_tokens::*, log_voter_info::*, unlock_tokens::*, update_voter_weight_record::*,
    withdraw::*,
};

mod claim;
mod close_deposit_entry;
mod close_voter;
mod configure_voting_mint;
mod create_deposit_entry;
mod create_registrar;
mod create_voter;
mod deposit;
mod extend_deposit;
mod lock_tokens;
mod log_voter_info;
mod unlock_tokens;
mod update_voter_weight_record;
mod withdraw;

pub fn clock_unix_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
