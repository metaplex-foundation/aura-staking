use anchor_lang::prelude::borsh;
use anchor_lang::Key;
use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use mplx_staking_states::state::LockupPeriod;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    system_program,
};

pub const REWARD_CONTRACT_ID: Pubkey =
    solana_program::pubkey!("J8oa8UUJBydrTKtCdkvwmQQ27ZFDq54zAxWJY5Ey72Ji");

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

/// Rewards initialize mining
#[allow(clippy::too_many_arguments)]
pub fn initialize_mining<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    user: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
) -> ProgramResult {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new(payer.key(), true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let ix =
        Instruction::new_with_borsh(*program_id, &RewardsInstruction::InitializeMining, accounts);

    invoke(&ix, &[reward_pool, mining, user, payer, system_program])
}

/// Rewards deposit mining
#[allow(clippy::too_many_arguments)]
pub fn deposit_mining<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    user: AccountInfo<'a>,
    deposit_authority: AccountInfo<'a>,
    amount: u64,
    lockup_period: LockupPeriod,
    signers_seeds: &[&[&[u8]]],
    reward_mint: &Pubkey,
) -> ProgramResult {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(*reward_mint, false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new_readonly(deposit_authority.key(), true),
    ];

    let ix = Instruction::new_with_borsh(
        *program_id,
        &RewardsInstruction::DepositMining {
            amount,
            lockup_period,
        },
        accounts,
    );

    invoke_signed(
        &ix,
        &[reward_pool, mining, user, deposit_authority],
        signers_seeds,
    )
}

/// Restake deposit
#[allow(clippy::too_many_arguments)]
pub fn extend_deposit<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    reward_mint: &Pubkey,
    user: AccountInfo<'a>,
    deposit_authority: AccountInfo<'a>,
    amount: u64,
    lockup_period: LockupPeriod,
    deposit_start_ts: u64,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(*reward_mint, false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new_readonly(deposit_authority.key(), true),
    ];

    let ix = Instruction::new_with_borsh(
        *program_id,
        &RewardsInstruction::RestakeDeposit {
            lockup_period,
            amount,
            deposit_start_ts,
        },
        accounts,
    );

    invoke_signed(
        &ix,
        &[reward_pool, mining, user, deposit_authority],
        signers_seeds,
    )?;

    Ok(())
}

/// Rewards withdraw mining
#[allow(clippy::too_many_arguments)]
pub fn withdraw_mining<'a>(
    program_id: &Pubkey,
    reward_pool: AccountInfo<'a>,
    mining: AccountInfo<'a>,
    user: AccountInfo<'a>,
    deposit_authority: AccountInfo<'a>,
    amount: u64,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let accounts = vec![
        AccountMeta::new(reward_pool.key(), false),
        AccountMeta::new(mining.key(), false),
        AccountMeta::new_readonly(user.key(), false),
        AccountMeta::new_readonly(deposit_authority.key(), true),
    ];

    let ix = Instruction::new_with_borsh(
        *program_id,
        &RewardsInstruction::WithdrawMining { amount },
        accounts,
    );

    invoke_signed(
        &ix,
        &[reward_pool, mining, user, deposit_authority],
        signers_seeds,
    )
}
