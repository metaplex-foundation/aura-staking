use {
    anchor_spl::token::TokenAccount,
    mplx_staking_states::state::{LockupKind, LockupPeriod},
    program_test::*,
    solana_program_test::*,
    solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError},
};

mod program_test;
#[tokio::test]
async fn test_all_deposits() -> Result<(), TransportError> {
    let context = TestContext::new().await;
    let addin = &context.addin;

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
    let voter_mngo = context.users[1].token_accounts[0];
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
    let mngo_voting_mint = addin
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

    let deposit_mining = find_deposit_mining_addr(
        &voter_authority.pubkey(),
        &rewards_pool,
        &context.rewards.program_id,
    );

    let voter = addin
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

    let delegate = Keypair::new();
    addin
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
        .await
        .unwrap();
    addin
        .deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            voter_mngo,
            0,
            32000,
        )
        .await
        .unwrap();

    for i in 1..32 {
        let delegate = Keypair::new();
        addin
            .create_deposit_entry(
                &registrar,
                &voter,
                voter_authority,
                &mngo_voting_mint,
                i,
                LockupKind::Constant,
                LockupPeriod::ThreeMonths,
                delegate.pubkey(),
            )
            .await
            .unwrap();
        addin
            .lock_tokens(
                &registrar,
                &voter,
                voter_authority,
                &deposit_mining,
                &context.rewards.program_id,
                0,
                i,
                1000,
                mngo_voting_mint.mint.pubkey.unwrap(),
                realm.realm,
            )
            .await?;
    }

    // advance time, to be in the middle of all deposit lockups
    advance_clock_by_ts(&mut context.solana.context.borrow_mut(), 45 * 86400).await;

    // the two most expensive calls which scale with number of deposits
    // are update_voter_weight_record and withdraw - both compute the vote weight

    let vwr = addin
        .update_voter_weight_record(&registrar, &voter)
        .await
        .unwrap();
    assert_eq!(vwr.voter_weight, 1000 * 32);

    advance_clock_by_ts(&mut context.solana.context.borrow_mut(), 50 * 86400).await;

    context
        .addin
        .unlock_tokens(&registrar, &voter, voter_authority, 0)
        .await
        .unwrap();

    advance_clock_by_ts(&mut context.solana.context.borrow_mut(), 5 * 86400).await;

    // make sure withdrawing works with all deposits filled
    addin
        .withdraw(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            voter_mngo,
            0,
            1000,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
        .await
        .unwrap();

    // logging can take a lot of cu/mem
    addin.log_voter_info(&registrar, &voter, 0).await;

    Ok(())
}
