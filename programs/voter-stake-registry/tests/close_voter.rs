use anchor_spl::token::TokenAccount;
use assert_custom_on_chain_error::AssertCustomOnChainErr;
use mplx_staking_states::{
    error::MplStakingError,
    state::{LockupKind, LockupPeriod},
};
use program_test::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};

mod program_test;

#[tokio::test]
async fn two_the_same_voting_mints_fail() -> Result<(), TransportError> {
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

    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            None,
            Some(&[context.mints[1].pubkey.unwrap()]),
        )
        .await;

    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            1,
            &context.mints[1],
            None,
            Some(&[context.mints[0].pubkey.unwrap()]),
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

    let balance_initial = voter.deposit_amount(&context.solana, 0).await;
    assert_eq!(balance_initial, 0);

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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    let mints = &[context.mints[0].clone(), context.mints[0].clone()];

    context
        .addin
        .close_voter(
            &registrar,
            &voter,
            &mints[..],
            voter_authority,
            &context.rewards.program_id,
        )
        .await
        .assert_on_chain_err(MplStakingError::InvalidAssoctiatedTokenAccounts);

    Ok(())
}

#[tokio::test]
async fn zero_ata_passed_instead_of_two() -> Result<(), TransportError> {
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

    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            None,
            Some(&[context.mints[1].pubkey.unwrap()]),
        )
        .await;

    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            1,
            &context.mints[1],
            None,
            Some(&[context.mints[0].pubkey.unwrap()]),
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

    let balance_initial = voter.deposit_amount(&context.solana, 0).await;
    assert_eq!(balance_initial, 0);

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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    let mints = &[];
    context
        .addin
        .close_voter(
            &registrar,
            &voter,
            &mints[..],
            voter_authority,
            &context.rewards.program_id,
        )
        .await
        .assert_on_chain_err(MplStakingError::InvalidAssoctiatedTokenAccounts);

    Ok(())
}

#[tokio::test]
async fn one_ata_passed_instead_of_two() -> Result<(), TransportError> {
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

    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            None,
            Some(&[context.mints[1].pubkey.unwrap()]),
        )
        .await;

    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            1,
            &context.mints[1],
            None,
            Some(&[context.mints[0].pubkey.unwrap()]),
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

    let balance_initial = voter.deposit_amount(&context.solana, 0).await;
    assert_eq!(balance_initial, 0);

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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    let mints = &[context.mints[0].clone()];
    context
        .addin
        .close_voter(
            &registrar,
            &voter,
            &mints[..],
            voter_authority,
            &context.rewards.program_id,
        )
        .await
        .assert_on_chain_err(MplStakingError::InvalidAssoctiatedTokenAccounts);

    Ok(())
}

#[tokio::test]
async fn success() -> Result<(), TransportError> {
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

    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            None,
            Some(&[context.mints[1].pubkey.unwrap()]),
        )
        .await;

    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            1,
            &context.mints[1],
            None,
            Some(&[context.mints[0].pubkey.unwrap()]),
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

    let balance_initial = voter.deposit_amount(&context.solana, 0).await;
    assert_eq!(balance_initial, 0);

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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    let lamports_before = context
        .solana
        .context
        .borrow_mut()
        .banks_client
        .get_balance(voter_authority.pubkey())
        .await?;

    context
        .addin
        .close_voter(
            &registrar,
            &voter,
            &context.mints[..],
            voter_authority,
            &context.rewards.program_id,
        )
        .await?;

    let lamports_after = context
        .solana
        .context
        .borrow_mut()
        .banks_client
        .get_balance(voter_authority.pubkey())
        .await?;
    assert!(lamports_after > lamports_before);

    Ok(())
}

#[tokio::test]
async fn wrong_order_of_passed_in_ata() -> Result<(), TransportError> {
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

    let mngo_voting_mint = context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            None,
            Some(&[context.mints[1].pubkey.unwrap()]),
        )
        .await;

    context
        .addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            1,
            &context.mints[1],
            None,
            Some(&[context.mints[0].pubkey.unwrap()]),
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

    let balance_initial = voter.deposit_amount(&context.solana, 0).await;
    assert_eq!(balance_initial, 0);

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

    // test deposit and withdraw
    let reference_account = context.users[1].token_accounts[0];
    context
        .addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    context
        .addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            deposit_authority,
            reference_account,
            0,
            10000,
        )
        .await?;

    let mints = &[context.mints[1].clone(), context.mints[0].clone()];
    context
        .addin
        .close_voter(
            &registrar,
            &voter,
            &mints[..],
            voter_authority,
            &context.rewards.program_id,
        )
        .await
        .assert_on_chain_err(MplStakingError::InvalidAssoctiatedTokenAccounts);

    Ok(())
}
