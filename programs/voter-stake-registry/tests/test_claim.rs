use std::str::FromStr;
use anchor_spl::token::TokenAccount;
use solana_program::msg;
use solana_program::pubkey::Pubkey;
use mpl_staking::state::{LockupKind, LockupPeriod};
use program_test::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use spl_governance::state::governance::GovernanceV2;
use spl_governance::state::proposal::ProposalV2;
use spl_governance::state::token_owner_record::get_token_owner_record_address;
use spl_governance::state::vote_record::{get_vote_record_address, VoteRecordV2};

mod program_test;

#[tokio::test]
async fn successeful_claim() -> Result<(), TransportError> {
    let context = TestContext::new().await;

    let payer = &context.users[0].key;
    let realm_authority = Keypair::new();
    let realm = context
        .governance
        .create_realm(
            "VSR Rewards 21",
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

    let rewards_source_ata = context.users[3].token_accounts[0];
    let amount = 100;
    let distribution_ends_at = context
        .solana
        .context
        .borrow_mut()
        .banks_client
        .get_sysvar::<solana_program::clock::Clock>()
        .await
        .unwrap()
        .unix_timestamp as u64
        + 86400;

    let reward_mint = &realm.community_token_mint.pubkey.unwrap();
    context
        .rewards
        .fill_vault(
            &rewards_pool,
            reward_mint,
            &fill_authority,
            &rewards_source_ata,
            amount,
            distribution_ends_at,
        )
        .await
        .unwrap();

    context
        .rewards
        .distribute_rewards(&rewards_pool, &distribution_authority)
        .await?;


    // create proposal `create_proposal`
    // vote for this proposal `cast_vote`
    let mint_governance = realm
        .create_mint_governance(
            context.mints[0].pubkey.unwrap(),
            &context.mints[0].authority,
            &voter,
            voter_authority,
            payer,
            context.addin.update_voter_weight_record_instruction(&registrar, &voter),
        )
        .await;

    let governance = &Pubkey::from_str("GovernanceProgramTest1111111111111111111111").unwrap();
    let proposal = realm.create_proposal(
        mint_governance.address,
        voter_authority,
        &voter,
        payer,
        context.addin.update_voter_weight_record_instruction(&registrar, &voter),
    ).await.unwrap();
    let _ = realm.cast_vote(
        mint_governance.address,
        &proposal,
        &voter,
        voter_authority,
        payer,
        context.addin.update_voter_weight_record_instruction(&registrar, &voter),
    ).await.unwrap();
    let vote_record = {
        let adrs = get_vote_record_address(
            &Pubkey::from_str("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw").unwrap(),
            &proposal.address,
            &proposal.owner_token_owner_record,
        );

        // let adrs = Pubkey::find_program_address(
        //     &[
        //         b"vote-record",
        //         proposal.address.as_ref(),
        //         voter_authority.as_ref(),
        //     ],
        //     &mint_governance.address, // SPL Governance Program ID
        // ).0;
        let vote_data = context.solana.get_account_data(adrs).await;
        let mut data_slice: &[u8] = &vote_data;
            let vote_record: spl_governance::state::vote_record::VoteRecordV2 =
            anchor_lang::AnchorDeserialize::deserialize(&mut data_slice).unwrap();
        // vote_record.governing_token_owner
        adrs
    };


    context
        .addin
        .claim(
            &rewards_pool,
            reward_mint,
            &deposit_mining,
            voter_authority,
            &voter_authority_ata,
            &context.rewards.program_id,
            &registrar,
            &mint_governance.address,
            &proposal.address,
            &vote_record,
        )
        .await?;

    let claimed_amount = context
        .solana
        .token_account_balance(voter_authority_ata)
        .await;
    assert_eq!(100, claimed_amount);

    Ok(())
}
