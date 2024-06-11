use anchor_spl::token::TokenAccount;
use mplx_staking_states::state::{LockupKind, LockupPeriod};
use program_test::*;
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transport::TransportError};
use std::{cell::RefCell, rc::Rc};

const DEPOSIT_A: u8 = 0;
const DEPOSIT_B: u8 = 1;

mod program_test;

struct LockupData {
    deposited: u64,
}

impl LockupData {
    pub fn new(_duration: u64, _time_left: u64, deposited: u64, _amount_unlocked: u64) -> Self {
        Self { deposited }
    }
}

async fn get_lockup_data(
    solana: &SolanaCookie,
    voter: Pubkey,
    index: u8,
    time_offset: i64,
) -> LockupData {
    let now = solana.get_clock().await.unix_timestamp + time_offset;
    let voter = solana
        .get_account::<mplx_staking_states::state::Voter>(voter)
        .await;
    let d = voter.deposits[index as usize];
    let duration = d.lockup.period.to_secs();
    LockupData::new(
        duration - d.lockup.seconds_left(now as u64),
        duration,
        d.amount_deposited_native,
        d.amount_deposited_native,
    )
}

#[tokio::test]
async fn test_internal_transfer_kind_of_none() -> Result<(), TransportError> {
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
    let mngo_voting_mint = addin
        .configure_voting_mint(
            &registrar,
            &realm_authority,
            payer,
            0,
            &context.mints[0],
            0,    // dump values, they doen't matter
            1.0,  // dump values, they doen't matter
            1.0,  // dump values, they doen't matter
            1,    // dump values, they doen't matter
            None, // dump values, they doen't matter
            None, // dump values, they doen't matter
        )
        .await;

    // TODO: ??? voter_authority == deposit_authority ???
    let voter_authority = deposit_authority;
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

    let reference_account = context.users[1].token_accounts[0];
    let deposit = |index: u8, amount: u64| {
        addin.deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            index,
            amount,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
    };

    let internal_transfer_unlocked = |source: u8, target: u8, amount: u64| {
        addin.internal_transfer_unlocked(
            &registrar,
            &voter,
            voter_authority,
            source,
            target,
            amount,
        )
    };
    let time_offset = Rc::new(RefCell::new(0i64));
    let lockup_status =
        |index: u8| get_lockup_data(&context.solana, voter.address, index, *time_offset.borrow());

    //
    // test transferring without any restrictions
    //
    addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            DEPOSIT_A,
            LockupKind::None,
            None,
            LockupPeriod::None,
        )
        .await
        .unwrap();
    deposit(DEPOSIT_A, 300).await.unwrap();

    addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            DEPOSIT_B,
            LockupKind::None,
            None,
            LockupPeriod::None,
        )
        .await
        .unwrap();

    //
    // test transfering unlocked funds
    //

    // successeful move
    internal_transfer_unlocked(DEPOSIT_A, DEPOSIT_B, 150).await?;
    // number is too hight because 300 - 150 < 300
    internal_transfer_unlocked(DEPOSIT_B, DEPOSIT_A, 300)
        .await
        .expect_err("amount too high");
    internal_transfer_unlocked(DEPOSIT_B, DEPOSIT_A, 10).await?;

    assert_eq!(lockup_status(DEPOSIT_A).await.deposited, 160);
    assert_eq!(lockup_status(DEPOSIT_B).await.deposited, 140);

    Ok(())
}

#[tokio::test]
async fn test_internal_transfer_kind_of_constant() -> Result<(), TransportError> {
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
            0,    // dump values, they don't matter
            1.0,  // dump values, they don't matter
            1.0,  // dump values, they don't matter
            1,    // dump values, they don't matter
            None, // dump values, they don't matter
            None, // dump values, they don't matter
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

    let reference_account = context.users[1].token_accounts[0];
    let deposit = |index: u8, amount: u64| {
        addin.deposit(
            &registrar,
            &voter,
            &mngo_voting_mint,
            voter_authority,
            reference_account,
            index,
            amount,
            &rewards_pool,
            &deposit_mining,
            &context.rewards.program_id,
        )
    };

    let internal_transfer_unlocked = |source: u8, target: u8, amount: u64| {
        addin.internal_transfer_unlocked(
            &registrar,
            &voter,
            voter_authority,
            source,
            target,
            amount,
        )
    };
    let time_offset = Rc::new(RefCell::new(0i64));
    let advance_time = |extra: u64| {
        *time_offset.borrow_mut() += extra as i64;
        addin.set_time_offset(&registrar, &realm_authority, *time_offset.borrow())
    };
    let lockup_status =
        |index: u8| get_lockup_data(&context.solana, voter.address, index, *time_offset.borrow());

    let month = 24 * 60 * 60 * 30;
    let day = 24 * 60 * 60;

    //
    // test transferring without any restrictions
    //
    addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            DEPOSIT_A,
            LockupKind::Constant,
            None,
            LockupPeriod::ThreeMonths,
        )
        .await
        .unwrap();
    deposit(DEPOSIT_A, 300).await.unwrap();

    addin
        .unlock_tokens(&registrar, &voter, voter_authority, DEPOSIT_A)
        .await
        .expect_err("try to unlock instanly should faild");

    addin
        .create_deposit_entry(
            &registrar,
            &voter,
            voter_authority,
            &mngo_voting_mint,
            DEPOSIT_B,
            LockupKind::None,
            None,
            LockupPeriod::None,
        )
        .await
        .unwrap();

    internal_transfer_unlocked(DEPOSIT_A, DEPOSIT_B, 300)
        .await
        .expect_err("still locked");

    // Advance slots to avoid caching of the UpdateVoterWeightRecord call
    // TODO: Is this something that could be an issue on a live node?
    context.solana.advance_clock_by_slots(2).await;
    advance_time(month * 3 + 60 * 60 * 6).await;

    internal_transfer_unlocked(DEPOSIT_A, DEPOSIT_B, 300)
        .await
        .expect_err("time has passed, but unlock hasn't been requested");

    // Advance slots to avoid caching of the UpdateVoterWeightRecord call
    // TODO: Is this something that could be an issue on a live node?
    context.solana.advance_clock_by_slots(2).await;
    advance_time(month * 3 + 60 * 60 * 6).await;
    addin
        .unlock_tokens(&registrar, &voter, voter_authority, DEPOSIT_A)
        .await?;

    // advance to cool down
    advance_time(day * 2).await;
    // cooled down, must work
    internal_transfer_unlocked(DEPOSIT_A, DEPOSIT_B, 300).await?;

    assert_eq!(lockup_status(DEPOSIT_A).await.deposited, 0);
    assert_eq!(lockup_status(DEPOSIT_B).await.deposited, 300);
    Ok(())
}
