use anchor_spl::token::TokenAccount;
use mplx_staking_states::state::{LockupKind, LockupPeriod};
use program_test::*;
use solana_program_test::*;
use solana_sdk::clock::SECONDS_PER_DAY;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn extend_from_flex() -> Result<(), TransportError> {
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

    let voter_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(voter_authority.pubkey(), payer)
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    let delegate = Keypair::new();
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
            LockupPeriod::OneYear,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            0,
            10000,
        )
        .await?;
    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter_authority,
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;
    advance_clock_by_ts(
        &mut context.solana.context.borrow_mut(),
        (SECONDS_PER_DAY * 365) as i64,
    )
    .await;

    context
        .addin
        .extend_deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            1,
            LockupPeriod::OneYear,
            0,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await?;

    let vault_balance = mngo_voting_mint
        .vault_balance(&context.solana, &voter)
        .await;
    let deposit_amount = voter.deposit_amount(&context.solana, 1).await;

    assert_eq!(vault_balance, 10000);
    assert_eq!(deposit_amount, 10000);

    Ok(())
}

#[tokio::test]
async fn extend_from_three_months_deposit() -> Result<(), TransportError> {
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

    let voter_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(voter_authority.pubkey(), payer)
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    let delegate = Keypair::new();
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
            LockupPeriod::OneYear,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            0,
            10000,
        )
        .await?;
    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter_authority,
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;
    advance_clock_by_ts(
        &mut context.solana.context.borrow_mut(),
        (SECONDS_PER_DAY * 365) as i64,
    )
    .await;

    context
        .addin
        .extend_deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            1,
            LockupPeriod::ThreeMonths,
            0,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await?;

    let vault_balance = mngo_voting_mint
        .vault_balance(&context.solana, &voter)
        .await;
    let deposit_amount = voter.deposit_amount(&context.solana, 1).await;

    assert_eq!(vault_balance, 10000);
    assert_eq!(deposit_amount, 10000);

    Ok(())
}

#[tokio::test]
async fn extend_deposit_after_one_year_for_three_months_with_top_up() -> Result<(), TransportError>
{
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

    let voter_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(voter_authority.pubkey(), payer)
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    let delegate = Keypair::new();
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
            LockupPeriod::OneYear,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            0,
            10000,
        )
        .await?;
    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter_authority,
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;
    advance_clock_by_ts(
        &mut context.solana.context.borrow_mut(),
        (SECONDS_PER_DAY * 365) as i64,
    )
    .await;

    context
        .addin
        .extend_deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            1,
            LockupPeriod::ThreeMonths,
            500,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await?;

    let vault_balance = mngo_voting_mint
        .vault_balance(&context.solana, &voter)
        .await;
    let deposit_amount = voter.deposit_amount(&context.solana, 1).await;

    assert_eq!(vault_balance, 10500);
    assert_eq!(deposit_amount, 10500);

    Ok(())
}

#[tokio::test]
async fn extend_from_flex_deposit_with_top_up() -> Result<(), TransportError> {
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

    let voter_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(voter_authority.pubkey(), payer)
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    let delegate = Keypair::new();
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
            LockupPeriod::OneYear,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            0,
            10000,
        )
        .await?;
    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter_authority,
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;
    advance_clock_by_ts(
        &mut context.solana.context.borrow_mut(),
        (SECONDS_PER_DAY * 365) as i64,
    )
    .await;

    context
        .addin
        .extend_deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            1,
            LockupPeriod::ThreeMonths,
            500,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await?;

    let deposit_amount = voter.deposit_amount(&context.solana, 1).await;

    assert_eq!(deposit_amount, 10500);

    Ok(())
}

#[tokio::test]
async fn extend_from_three_month_to_one_year() -> Result<(), TransportError> {
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

    let voter_authority = &context.users[1].key;
    let token_owner_record = realm
        .create_token_owner_record(voter_authority.pubkey(), payer)
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    let delegate = Keypair::new();
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            0,
            LockupKind::None,
            LockupPeriod::None,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            1,
            LockupKind::Constant,
            LockupPeriod::ThreeMonths,
            delegate.pubkey(),
        )
        .await?;
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            0,
            10000,
        )
        .await?;
    context
        .addin
        .stake(
            &registrar,
            &voter,
            voter_authority,
            &context.rewards.program_id,
            0,
            1,
            10000,
        )
        .await?;
    advance_clock_by_ts(
        &mut context.solana.context.borrow_mut(),
        (SECONDS_PER_DAY * 10) as i64,
    )
    .await;

    context
        .addin
        .extend_deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            1,
            LockupPeriod::OneYear,
            500,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await
        .expect_err("Impossible to extend stake from existing stake (not flex) to another period");

    Ok(())
}