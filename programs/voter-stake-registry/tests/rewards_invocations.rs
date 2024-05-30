use anchor_spl::token::TokenAccount;
use mplx_staking_states::state::LockupPeriod;
use program_test::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use voter_stake_registry::cpi_instructions::deposit_mining;

use crate::rewards::{AccountType, RewardsRoot};

pub mod program_test;

#[tokio::test]
pub async fn initialize_root() -> std::result::Result<(), TransportError> {
    let context = TestContext::new().await;
    let payer = &context.users[0].key;

    let rewards_root_kp = context.rewards.initialize_root(payer).await?;

    let rewards_root_account = context
        .solana
        .get_account::<RewardsRoot>(rewards_root_kp.pubkey())
        .await;

    assert_eq!(rewards_root_account.authority, payer.pubkey());
    assert_eq!(rewards_root_account.account_type, AccountType::RewardsRoot);

    Ok(())
}

// just run transaction to make sure they work
#[tokio::test]
pub async fn initialize_rewards_flow() -> std::result::Result<(), TransportError> {
    // let context = TestContext::new().await;
    // let payer = &context.users[0].key;

    // // create token mint
    // let reward_mint = Keypair::new();
    // let manager = &payer.pubkey();
    // create_mint(
    //     &mut context.solana.context.borrow_mut(),
    //     &reward_mint,
    //     manager,
    // )
    // .await
    // .unwrap();

    // let rewards_root_kp = context.rewards.initialize_root(payer).await?;

    // let deposit_authority = Keypair::new();
    // let rewards_pool = context
    //     .rewards
    //     .initialize_pool(&rewards_root_kp, &deposit_authority, payer)
    //     .await?;

    // let _vault = context
    //     .rewards
    //     .add_vault(
    //         &rewards_root_kp.pubkey(),
    //         &rewards_pool,
    //         &reward_mint.pubkey(),
    //         payer,
    //     )
    //     .await?;

    // let user = Keypair::new();

    // let _mining = context
    //     .rewards
    //     .initialize_mining(&rewards_pool, &user.pubkey(), payer)
    //     .await?;

    // let amount = 1;
    // let lockup_period = LockupPeriod::ThreeMonths;
    // context
    //     .rewards
    //     .deposit_mining(
    //         &rewards_pool,
    //         &user.pubkey(),
    //         &deposit_authority,
    //         amount,
    //         lockup_period,
    //         &reward_mint.pubkey(),
    //     )
    //     .await?;

    // TODO: will not work because no deposits yet
    // let rewarder = context
    //     .solana
    //     .create_token_account(&payer.pubkey(), reward_mint.pubkey())
    //     .await;
    // let amount = 100;
    // context
    //     .rewards
    //     .fill_vault(
    //         &rewards_pool,
    //         &reward_mint.pubkey(),
    //         &vault,
    //         payer,
    //         &rewarder,
    //         amount,
    //     )
    //     .await?;

    Ok(())
}
