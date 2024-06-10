use std::rc::Rc;

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
    pub solana: Rc<SolanaCookie>,
    pub program_id: Pubkey,
}

impl RewardsCookie {
    pub async fn initialize_pool(
        &self,
        deposit_authority: &Pubkey,
        fill_authority: &Pubkey,
        distribution_authority: &Pubkey,
        payer: &Keypair,
        reward_mint: &Pubkey,
    ) -> std::result::Result<(Pubkey, Pubkey), BanksClientError> {
        let (reward_pool, _reward_pool_bump) = Pubkey::find_program_address(
            &[
                "reward_pool".as_bytes(),
                &deposit_authority.key().to_bytes(),
                &fill_authority.key().to_bytes(),
            ],
            &self.program_id,
        );

        let (reward_vault, _reward_vault_bump) = Pubkey::find_program_address(
            &[
                "vault".as_bytes(),
                &reward_pool.key().to_bytes(),
                &reward_mint.key().to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(reward_pool, false),
            AccountMeta::new_readonly(*reward_mint, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::InitializePool {
                deposit_authority: *deposit_authority,
                fill_authority: *fill_authority,
                distribution_authority: *distribution_authority,
            },
            accounts,
        );

        let signers = vec![payer];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok((reward_pool, reward_vault))
    }

    pub async fn fill_vault(
        &self,
        reward_pool: &Pubkey,
        reward_mint: &Pubkey,
        fill_authority: &Keypair,
        source_token_account: &Pubkey,
        amount: u64,
        distribution_ends_at: u64,
    ) -> std::result::Result<(), BanksClientError> {
        let (vault, _bump) = Pubkey::find_program_address(
            &[
                "vault".as_bytes(),
                &reward_pool.to_bytes(),
                &reward_mint.to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new_readonly(*reward_mint, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(fill_authority.pubkey(), true),
            AccountMeta::new(*source_token_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::FillVault {
                amount,
                distribution_ends_at,
            },
            accounts,
        );

        let signers = vec![fill_authority];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(())
    }

    pub async fn distribute_rewards(
        &self,
        reward_pool: &Pubkey,
        reward_mint: &Pubkey,
        reward_vault: &Pubkey,
        distribute_authority: &Keypair,
    ) -> std::result::Result<(), BanksClientError> {
        let (vault, _bump) = Pubkey::find_program_address(
            &[
                "vault".as_bytes(),
                &reward_pool.to_bytes(),
                &reward_mint.to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new_readonly(*reward_mint, false),
            AccountMeta::new(*reward_vault, false),
            AccountMeta::new_readonly(distribute_authority.pubkey(), true),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::DistributeRewards,
            accounts,
        );

        let signers = vec![distribute_authority];

        self.solana
            .process_transaction(&[ix], Some(&signers))
            .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn initialize_mining<'a>(
        &self,
        reward_pool: &Pubkey,
        mining_owner: &Pubkey,
        payer: &Keypair,
    ) -> std::result::Result<Pubkey, BanksClientError> {
        let (mining, _bump) = Pubkey::find_program_address(
            &[
                "mining".as_bytes(),
                &mining_owner.key().to_bytes(),
                &reward_pool.key().to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new(mining, false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::InitializeMining {
                mining_owner: *mining_owner,
            },
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
        deposit_authority: &Keypair,
        amount: u64,
        lockup_period: LockupPeriod,
        owner: &Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let (mining, _bump) = Pubkey::find_program_address(
            &[
                "mining".as_bytes(),
                &owner.key().to_bytes(),
                &reward_pool.key().to_bytes(),
            ],
            &self.program_id,
        );

        let accounts = vec![
            AccountMeta::new(*reward_pool, false),
            AccountMeta::new(mining, false),
            AccountMeta::new_readonly(deposit_authority.pubkey(), true),
        ];

        let ix = Instruction::new_with_borsh(
            self.program_id,
            &RewardsInstruction::DepositMining {
                amount,
                lockup_period,
                owner: *owner,
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
