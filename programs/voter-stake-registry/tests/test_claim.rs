use anchor_spl::token::TokenAccount;
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transport::TransportError};

use mplx_staking_states::state::{LockupKind, LockupPeriod};
use program_test::*;

mod program_test;

#[tokio::test]
async fn successeful_claim() -> Result<(), TransportError> {
    let context = TestContext::new().await;

    let payer = &context.users[0].key;
    let realm_authority = Keypair::new();
    let realm = context
        .governance
        .create_realm(
            "testrealm",
            realm_authority.pubkey(),
            &context.mints[0],
            payer,
            &context.addin.program_id,
        )
        .await;

    let deposit_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(deposit_authority.pubkey(), payer)
        .await;

    let registrar = context
        .addin
        .create_registrar(&realm, &realm_authority, payer)
        .await;
    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            10,
            0.0,
            0.0,
            1,
            None,
            None,
        )
        .await;
    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            0,
            1.0,
            0.0,
            5 * 365 * 24 * 60 * 60,
            None,
            None,
        )
        .await;

    let rewards_pool = initialize_rewards_contract(
        payer,
        &context,
        &mngo_voting_mint.mint.pubkey.unwrap(),
        &deposit_authority.pubkey(),
    )
    .await?;

    // TODO: ??? voter_authority == deposit_authority ???
    let voter_authority = deposit_authority;
    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
    );
    let voter_authority_ata = context
        .rewards
        .solana
        .create_spl_ata(
            &voter_authority.pubkey(),
            &mngo_voting_mint.mint.pubkey.unwrap(),
            payer,
        )
        .await;
    let voter = context
        .addin
        .create_voter(
            &registrar,
            &token_owner_record,
            voter_authority,
            payer,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await;

    let depositer_token_account = context.users[1].token_accounts[0];

    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::Constant,
            None,
            LockupPeriod::ThreeMonths,
        )
        .await?;

    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            depositer_token_account,
            0,
            10000,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await?;

    let rewards_source_ata = context.users[0].token_accounts[0];
    context
        .rewards
        .fill_vault(
            &rewards_pool,
            &mngo_voting_mint.mint.pubkey.unwrap(),
            payer,
            &rewards_source_ata,
            100,
        )
        .await
        .unwrap();

    context
        .addin
        .claim(
            &rewards_pool,
            &mngo_voting_mint.mint.pubkey.unwrap(),
            &deposit_mining,
            voter_authority,
            &voter_authority_ata,
            &context.rewards.program_id,
        )
        .await?;

    let claimed_amount = context
        .solana
        .token_account_balance(voter_authority_ata)
        .await;
    assert_eq!(100, claimed_amount);

    Ok(())
}
