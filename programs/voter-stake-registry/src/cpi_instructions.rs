use anchor_lang::prelude::borsh;
use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use mplx_staking_states::state::LockupPeriod;

// TODO: move the const
pub const REWARD_CONTRACT_ID: Pubkey = solana_program::pubkey!("11111111111111111111111111111111");

#[derive(Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum RewardsInstruction {
    /// Creates and initializes a reward pool account
    ///
    /// Accounts:
    /// [R] Root account (ex-Config program account)
    /// [W] Reward pool account
    /// [R] Deposit authority
    /// [WS] Payer
    /// [R] System program
    InitializePool,

    /// Creates a new vault account and adds it to the reward pool
    ///
    /// Accounts:
    /// [R] Root account (ex-Config program account)
    /// [W] Reward pool account
    /// [R] Reward mint account
    /// [W] Vault account
    /// [WS] Payer
    /// [R] Token program
    /// [R] System program
    /// [R] Rent sysvar
    AddVault,

    /// Fills the reward pool with rewards
    ///
    /// Accounts:
    /// [W] Reward pool account
    /// [R] Mint of rewards account
    /// [W] Vault for rewards account
    /// [RS] Transfer  account
    /// [W] From account
    /// [R] Token program
    FillVault {
        /// Amount to fill
        amount: u64,
    },

    /// Initializes mining account for the specified user
    ///
    /// Accounts:
    /// [W] Reward pool account
    /// [W] Mining
    /// [R] User
    /// [WS] Payer
    /// [R] System program
    /// [R] Rent sysvar
    InitializeMining,

    /// Deposits amount of supply to the mining account
    ///
    /// Accounts:
    /// [W] Reward pool account
    /// [W] Mining
    /// [R] Mint of rewards account
    /// [R] User
    /// [RS] Deposit authority
    DepositMining {
        /// Amount to deposit
        amount: u64,
        /// Lockup Period
        lockup_period: LockupPeriod,
    },

    /// Withdraws amount of supply to the mining account
    ///
    /// Accounts:
    /// [W] Reward pool account
    /// [W] Mining
    /// [R] User
    /// [RS] Deposit authority
    WithdrawMining {
        /// Amount to withdraw
        amount: u64,
    },

    /// Claims amount of rewards
    ///
    /// Accounts:
    /// [R] Reward pool account
    /// [R] Mint of rewards account
    /// [W] Vault for rewards account
    /// [W] Mining
    /// [RS] User
    /// [W] User reward token account
    /// [R] Token program
    Claim,

    /// Creates and initializes a reward root
    ///
    /// Accounts:
    /// [WS] Root account (ex-Config program account)
    /// [WS] Authority
    /// [R] System program
    InitializeRoot,

    /// Restakes deposit
    ///
    /// Accounts:
    /// [W] Reward pool account
    /// [W] Mining
    /// [R] Mint of rewards account
    /// [R] User
    /// [RS] Deposit authority
    RestakeDeposit {
        /// Requested lockup period for restaking
        lockup_period: LockupPeriod,
        /// Amount of tokens to be restaked
        amount: u64,
        /// Deposit start_ts
        deposit_start_ts: u64,
    },
}
