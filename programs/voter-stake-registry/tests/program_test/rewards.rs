use std::sync::Arc;

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use mplx_staking_states::state::LockupPeriod;
use solana_program_test::*;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::sysvar;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
};
use voter_stake_registry::cpi_instructions::RewardsInstruction;

use crate::SolanaCookie;

pub struct RewardsCookie {
    pub solana: Arc<SolanaCookie>,
    pub program_id: Pubkey,
}

impl RewardsCookie {
    pub async fn initialize_root(
        &self,
        payer: &Keypair,
    ) -> std::result::Result<Keypair, BanksClientError> {
        let rewards_root = Keypair::new();

        let accounts = vec![
            AccountMeta::new(rewards_root.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::InitializeRoot,
            accounts,
        );

        let signers = vec![payer, &rewards_root];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(rewards_root)
    }

    pub async fn initialize_pool(
        &self,
        rewards_root: &Keypair,
        deposit_authority: &Keypair,
        payer: &Keypair,
    ) -> std::result::Result<Pubkey, BanksClientError> {
        let (reward_pool, _bump) = Pubkey::find_program_address(
            &["reward_pool".as_bytes(), &rewards_root.pubkey().to_bytes()],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new_readonly(rewards_root.pubkey(), false),
            AccountMeta::new(reward_pool, false),
            AccountMeta::new_readonly(deposit_authority.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::InitializePool,
            accounts,
        );

        let signers = vec![payer];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(reward_pool)
    }

    pub async fn add_vault(
        &self,
        rewards_root: &Pubkey,
        reward_pool: &Pubkey,
        reward_mint: &Pubkey,
        payer: &Keypair,
    ) -> std::result::Result<Pubkey, BanksClientError> {
        let (vault, _bump) = Pubkey::find_program_address(
            &[
                "vault".as_bytes(),
                &reward_pool.to_bytes(),
                &reward_mint.to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new_readonly(*rewards_root, false),
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new_readonly(*reward_mint, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];

        let ix =
            Instruction::new_with_borsh(self.program_id, &RewardsInstruction::AddVault, accounts);

        let signers = vec![payer];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(vault)
    }

    pub async fn fill_vault(
        &self,
        reward_pool: &Pubkey,
        reward_mint: &Pubkey,
        vault: &Pubkey,
        authority: &Keypair,
        source_token_account: &Pubkey,
        amount: u64,
    ) -> std::result::Result<(), BanksClientError> {
        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new_readonly(*reward_mint, false),
            AccountMeta::new(*vault, false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new(*source_token_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::FillVault { amount },
            accounts,
        );

        let signers = vec![authority];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn initialize_mining<'a>(
        &self,
        reward_pool: &Pubkey,
        user: &Pubkey,
        payer: &Keypair,
    ) -> std::result::Result<Pubkey, BanksClientError> {
        let (mining, _bump) = Pubkey::find_program_address(
            &[
                "mining".as_bytes(),
                &user.key().to_bytes(),
                &reward_pool.key().to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new(mining, false),
            AccountMeta::new_readonly(*user, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::InitializeMining,
            accounts,
        );

        let signers = vec![payer];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(mining)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn deposit_mining<'a>(
        &self,
        reward_pool: &Pubkey,
        user: &Pubkey,
        deposit_authority: &Keypair,
        amount: u64,
        lockup_period: LockupPeriod,
        mint_account: &Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let (mining, _bump) = Pubkey::find_program_address(
            &[
                "mining".as_bytes(),
                &user.key().to_bytes(),
                &reward_pool.key().to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new(mining, false),
            AccountMeta::new_readonly(*user, false),
            AccountMeta::new_readonly(deposit_authority.pubkey(), true),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::DepositMining {
                amount,
                lockup_period,
                reward_mint_addr: *mint_account,
            },
            accounts,
        );

        let signers = vec![deposit_authority];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Default)]
pub struct RewardsRoot {
    /// Account type - RewardsRoot
    pub account_type: AccountType,
    /// Authority address
    pub authority: Pubkey,
}

#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Default)]
pub enum AccountType {
    /// If the account has not been initialized, the enum will be 0
    #[default]
    Uninitialized,
    /// Rewards root
    RewardsRoot,
    /// Reward pool
    RewardPool,
}

impl IsInitialized for RewardsRoot {
    fn is_initialized(&self) -> bool {
        self.account_type != AccountType::Uninitialized
    }
}

impl AccountDeserialize for RewardsRoot {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let rewards_root: RewardsRoot =
            AnchorDeserialize::deserialize(buf).map_err(|_| ErrorCode::AccountDidNotDeserialize)?;
        if !IsInitialized::is_initialized(&rewards_root) {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(rewards_root)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let rewards_root: RewardsRoot = AnchorDeserialize::deserialize(buf)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
        Ok(rewards_root)
    }
}
