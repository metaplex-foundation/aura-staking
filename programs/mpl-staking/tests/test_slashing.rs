use anchor_spl::token::TokenAccount;
use mpl_common_constants::constants::REALM_NAME;
use mplx_staking_states::state::{LockupKind, LockupPeriod};
use program_test::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};

mod program_test;

#[tokio::test]
async fn slash_success() -> Result<(), TransportError> {
    let context = TestContext::new().await;

    let payer = &context.users[0].key;
    let realm_authority = Keypair::new();
    let realm = context
        .governance
        .create_realm(
            REALM_NAME,
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

    let fill_authority = Keypair::from_bytes(&context.users[3].key.to_bytes()).unwrap();
    let distribution_authority = Keypair::new();
    let (registrar, rewards_pool) = context
        .addin
        .create_registrar(
            &realm,
            &realm_authority,
            payer,
            &fill_authority.pubkey(),
            &distribution_authority.pubkey(),
            &context.rewards.program_id,
        )
        .await;
    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
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
            None,
            None,
        )
        .await;

    // TODO: ??? voter_authority == deposit_authority ???
    let voter_authority = deposit_authority;
    let (deposit_mining, _) = find_deposit_mining_addr(
        &context.rewards.program_id,
        &voter_authority.pubkey(),
        &rewards_pool,
    );
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
            &voter,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            &voter,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
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
        )
        .await?;

    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter.authority.pubkey(),
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;

    context
        .addin
        .slash(
            &registrar,
            &voter,
            &realm_authority,
            1,
            5000,
            &voter_authority.pubkey(),
            &context.rewards.program_id,
        )
        .await
        .unwrap();

    let voter_authority_ata = context
        .rewards
        .solana
        .create_spl_ata(
            &voter_authority.pubkey(),
            &mngo_voting_mint.mint.pubkey.unwrap(),
            payer,
        )
        .await;

    advance_clock_by_ts(&mut context.solana.context.borrow_mut(), 90 * 86400).await;
    context
        .addin
        .unlock_tokens(
            &registrar,
            &voter,
            &voter,
            1,
            &rewards_pool,
            &context.rewards.program_id,
        )
        .await?;

    advance_clock_by_ts(&mut context.solana.context.borrow_mut(), 5 * 86400).await;
    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            voter_authority_ata,
            realm.community_token_account,
            1,
            5000,
        )
        .await?;

    let claimed_amount = context
        .solana
        .token_account_balance(realm.community_token_account)
        .await;
    assert_eq!(5000, claimed_amount);

    Ok(())
}
