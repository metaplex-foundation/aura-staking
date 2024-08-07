use crate::{borsh::BorshDeserialize, cpi_instructions};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use mplx_staking_states::{error::MplStakingError, state::Registrar};
use solana_program::program::get_return_data;
use spl_governance::state::{
    legacy::GovernanceV1, proposal::ProposalV2, realm::RealmV2, vote_record::VoteRecordV2,
};
use std::{borrow::Borrow, slice::Iter, str::FromStr};

pub const DAO_PUBKEY: &str = "89wVNeyqqDaWKWtS4rbunYdsxxbe5V3VRx6g8GWNMTMt";

#[derive(Accounts)]
pub struct Claim<'info> {
    /// CHECK:
    /// Ownership of the account will be checked in the rewards contract
    /// It's the core account for the rewards contract, which will
    /// keep track of all rewards and staking logic.
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: Rewards mint addr will be checked in the rewards contract
    pub reward_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Rewards vault is used as a source of rewards and
    /// is checked on the rewards contract
    /// PDA(["vault", reward_pool, reward_mint], reward_program)
    pub vault: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    /// PDA(["mining", mining owner <aka voter_authority in our case>, reward_pool],
    /// reward_program)
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    pub mining_owner: Signer<'info>,

    /// CHECK: Registrar plays the role of deposit_authority on the Rewards Contract,
    /// therefore their PDA that should sign the CPI call
    pub registrar: AccountLoader<'info, Registrar>,

    #[account(mut)]
    pub user_reward_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Rewards Program account
    #[account(executable)]
    pub rewards_program: UncheckedAccount<'info>,
}

/// Claims token from the Rewards Contract.
///
/// Tokens will be transfered from Vault in Rewards account to User's user_reward_token_account.
/// This call actually doesn't mutating Staking's accounts, only Reward's accounts will be mutated.
pub fn claim(
    ctx: Context<Claim>,
    registrar_bump: u8,
    realm_governing_mint_pubkey: Pubkey,
    realm_pubkey: Pubkey,
) -> Result<u64> {
    let mut remaining_accounts_iter = ctx.remaining_accounts.iter();

    let (_, realm_addr) = deserialize::<RealmV2>(&mut remaining_accounts_iter)?;
    let (governance, governance_addr) = deserialize::<GovernanceV1>(&mut remaining_accounts_iter)?;
    let (proposal, _) = deserialize::<ProposalV2>(&mut remaining_accounts_iter)?;
    let (vote_record, _) = deserialize::<VoteRecordV2>(&mut remaining_accounts_iter)?;

    require!(
        realm_addr == Pubkey::from_str(DAO_PUBKEY).unwrap()
            && governance.realm == realm_addr
            && proposal.governance == governance_addr
            && vote_record.governing_token_owner == *ctx.accounts.mining_owner.key,
        MplStakingError::NoDaoInteractionFound
    );

    let registrar = ctx.accounts.registrar.load()?;

    require!(
        registrar.reward_pool == ctx.accounts.reward_pool.key(),
        MplStakingError::InvalidRewardPool
    );

    let rewards_program = ctx.accounts.rewards_program.to_account_info();
    let reward_pool = ctx.accounts.reward_pool.to_account_info();
    let rewards_mint = ctx.accounts.reward_mint.to_account_info();
    let vault = ctx.accounts.vault.to_account_info();
    let deposit_mining = ctx.accounts.deposit_mining.to_account_info();
    let deposit_authority = ctx.accounts.registrar.to_account_info();
    let mining_owner = ctx.accounts.mining_owner.to_account_info();
    let user_reward_token_account = ctx.accounts.user_reward_token_account.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let signers_seeds = &[
        &realm_pubkey.key().to_bytes(),
        b"registrar".as_ref(),
        &realm_governing_mint_pubkey.key().to_bytes(),
        &[registrar_bump][..],
    ];

    cpi_instructions::claim(
        rewards_program,
        reward_pool,
        rewards_mint,
        vault,
        deposit_mining,
        mining_owner,
        deposit_authority,
        user_reward_token_account,
        token_program,
        signers_seeds,
    )?;

    if let Some((_rewards_program_id, claimed_rewards_raw)) = get_return_data() {
        let claimed_rewards = u64::deserialize(&mut claimed_rewards_raw.borrow())?;
        msg!("Rewards are clamed {:?}", claimed_rewards);
        Ok(claimed_rewards)
    } else {
        Err(MplStakingError::CpiReturnDataIsAbsent.into())
    }
}

fn deserialize<T: BorshDeserialize>(iter: &mut Iter<AccountInfo>) -> Result<(T, Pubkey)> {
    let item = iter
        .next()
        .ok_or(MplStakingError::RemainingAccountsIsNotLongEnough)?;
    let addr = *item.key;
    let obj = T::deserialize(&mut &item.try_borrow_mut_data()?[..])?;
    Ok((obj, addr))
}
