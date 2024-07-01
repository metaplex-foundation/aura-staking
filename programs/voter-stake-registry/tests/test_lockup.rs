use anchor_spl::token::TokenAccount;
use mplx_staking_states::state::{LockupKind, LockupPeriod};
use program_test::*;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_unlock_and_withdraw_before_end_ts() -> Result<(), TransportError> {
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
        .lock_tokens(
            &registrar,
            &voter,
            voter_authority,
            &deposit_mining,
            &context.rewards.program_id,
            0,
            1,
            10000,
            mngo_voting_mint.mint.pubkey.unwrap(),
            realm.realm,
        )
        .await?;

    context
        .addin
        .unlock_tokens(&registrar, &voter, voter_authority, 1)
        .await
        .expect_err("fails because it's too early to unlock is invalid");
    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            &context.users[1].key,
            reference_account,
            1,
            10000,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await
        .expect_err("fails because it's impossible to withdraw without unlock");

    Ok(())
}

#[tokio::test]
async fn test_unlock_after_end_ts() -> Result<(), TransportError> {
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
        .lock_tokens(
            &registrar,
            &voter,
            voter_authority,
            &deposit_mining,
            &context.rewards.program_id,
            0,
            1,
            10000,
            mngo_voting_mint.mint.pubkey.unwrap(),
            realm.realm,
        )
        .await?;

    // advance to 365 days
    let secs_per_day = 24 * 60 * 60;
    context
        .addin
        .set_time_offset(&registrar, &realm_authority, 365 * secs_per_day)
        .await;

    // unlock is possible
    context
        .addin
        .unlock_tokens(&registrar, &voter, voter_authority, 1)
        .await
        .unwrap();

    // unlocked, but cooldown hasn't passed yet
    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            &context.users[1].key,
            reference_account,
            1,
            10000,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await
        .expect_err("fails because cooldown is ongoing");

    Ok(())
}

#[tokio::test]
async fn test_unlock_and_withdraw_after_end_ts_and_cooldown() -> Result<(), TransportError> {
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
        .lock_tokens(
            &registrar,
            &voter,
            voter_authority,
            &deposit_mining,
            &context.rewards.program_id,
            0,
            1,
            10000,
            mngo_voting_mint.mint.pubkey.unwrap(),
            realm.realm,
        )
        .await?;
    let secs_per_day = 24 * 60 * 60;
    // advance to day 365
    context
        .addin
        .set_time_offset(&registrar, &realm_authority, 365 * secs_per_day)
        .await;

    context
        .addin
        .unlock_tokens(&registrar, &voter, voter_authority, 1)
        .await
        .unwrap();

    // advance to day 370 (one year + cooldown (5 days))
    context
        .addin
        .set_time_offset(&registrar, &realm_authority, 370 * secs_per_day)
        .await;

    // withdraw must be successful
    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            &context.users[1].key,
            reference_account,
            1,
            10000,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await
        .unwrap();

    Ok(())
}
