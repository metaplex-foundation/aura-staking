use anchor_spl::token::TokenAccount;
use assert_custom_on_chain_error::AssertCustomOnChainErr;
use mpl_common_constants::constants::{GOVERNANCE_PROGRAM_ID, REALM_NAME};
use mplx_staking_states::{
    error::MplStakingError,
    state::{LockupKind, LockupPeriod},
};
use program_test::*;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use spl_governance::state::vote_record::get_vote_record_address;

mod program_test;

struct CLaimSetup {
    context: TestContext,
    realm: GovernanceRealmCookie,
    mint_governance: MintGovernanceCookie,
    proposal: ProposalCookie,
    voter: VoterCookie,
    voter_authority: Keypair,
    registrar: RegistrarCookie,
    rewards_pool: Pubkey,
    reward_mint: Pubkey,
    deposit_mining: Pubkey,
    voter_authority_ata: Pubkey,
    realm_authority: Keypair,
    payer: Keypair,
}

async fn setup(realm_name: &str) -> Result<CLaimSetup, TransportError> {
    let context = TestContext::new().await;

    let payer = &context.users[0].key;
    let realm_authority = Keypair::new();
    let realm = context
        .governance
        .create_realm(
            realm_name,
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

    let mint_governance = realm
        .create_mint_governance(
            context.mints[0].pubkey.unwrap(),
            &context.mints[0].authority,
            &voter,
            voter_authority,
            payer,
            context
                .addin
                .update_voter_weight_record_instruction(&registrar, &voter),
        )
        .await;

    let proposal = realm
        .create_proposal(
            mint_governance.address,
            voter_authority,
            &voter,
            payer,
            context
                .addin
                .update_voter_weight_record_instruction(&registrar, &voter),
        )
        .await
        .unwrap();

    Ok(CLaimSetup {
        voter_authority: clone_keypair(voter_authority),
        payer: clone_keypair(payer),
        context,
        realm,
        mint_governance,
        proposal,
        voter,
        registrar,
        rewards_pool,
        reward_mint: *reward_mint,
        deposit_mining,
        voter_authority_ata,
        realm_authority,
    })
}

#[tokio::test]
async fn successeful_claim() -> Result<(), TransportError> {
    let claim_setup = setup(REALM_NAME).await?;

    claim_setup
        .realm
        .cast_vote(
            claim_setup.mint_governance.address,
            &claim_setup.proposal,
            &claim_setup.voter,
            &claim_setup.voter_authority,
            &claim_setup.payer,
            claim_setup
                .context
                .addin
                .update_voter_weight_record_instruction(&claim_setup.registrar, &claim_setup.voter),
        )
        .await
        .unwrap();
    let vote_record = get_vote_record_address(
        &Pubkey::from(GOVERNANCE_PROGRAM_ID),
        &claim_setup.proposal.address,
        &claim_setup.proposal.owner_token_owner_record,
    );

    claim_setup
        .context
        .addin
        .claim(
            &claim_setup.rewards_pool,
            &claim_setup.reward_mint,
            &claim_setup.deposit_mining,
            &claim_setup.voter_authority,
            &claim_setup.voter_authority_ata,
            &claim_setup.context.rewards.program_id,
            &claim_setup.registrar,
            &claim_setup.mint_governance.address,
            &claim_setup.proposal.address,
            &claim_setup.voter,
            &vote_record,
        )
        .await?;

    let claimed_amount = claim_setup
        .context
        .solana
        .token_account_balance(claim_setup.voter_authority_ata)
        .await;
    assert_eq!(100, claimed_amount);

    Ok(())
}

#[tokio::test]
async fn claim_is_restricted() -> Result<(), TransportError> {
    let claim_setup = setup(REALM_NAME).await?;

    claim_setup
        .realm
        .cast_vote(
            claim_setup.mint_governance.address,
            &claim_setup.proposal,
            &claim_setup.voter,
            &claim_setup.voter_authority,
            &claim_setup.payer,
            claim_setup
                .context
                .addin
                .update_voter_weight_record_instruction(&claim_setup.registrar, &claim_setup.voter),
        )
        .await
        .unwrap();
    let vote_record = get_vote_record_address(
        &Pubkey::from(GOVERNANCE_PROGRAM_ID),
        &claim_setup.proposal.address,
        &claim_setup.proposal.owner_token_owner_record,
    );

    claim_setup
        .context
        .addin
        .restrict_tokenflow(
            &claim_setup.registrar,
            &claim_setup.realm_authority,
            &claim_setup.voter,
        )
        .await
        .unwrap();

    claim_setup
        .context
        .addin
        .claim(
            &claim_setup.rewards_pool,
            &claim_setup.reward_mint,
            &claim_setup.deposit_mining,
            &claim_setup.voter_authority,
            &claim_setup.voter_authority_ata,
            &claim_setup.context.rewards.program_id,
            &claim_setup.registrar,
            &claim_setup.mint_governance.address,
            &claim_setup.proposal.address,
            &claim_setup.voter,
            &vote_record,
        )
        .await
        .expect_err("Claiming is restricted by Rewards program");

    Ok(())
}

#[tokio::test]
async fn claim_is_allowed() -> Result<(), TransportError> {
    let claim_setup = setup(REALM_NAME).await?;

    claim_setup
        .realm
        .cast_vote(
            claim_setup.mint_governance.address,
            &claim_setup.proposal,
            &claim_setup.voter,
            &claim_setup.voter_authority,
            &claim_setup.payer,
            claim_setup
                .context
                .addin
                .update_voter_weight_record_instruction(&claim_setup.registrar, &claim_setup.voter),
        )
        .await
        .unwrap();
    let vote_record = get_vote_record_address(
        &Pubkey::from(GOVERNANCE_PROGRAM_ID),
        &claim_setup.proposal.address,
        &claim_setup.proposal.owner_token_owner_record,
    );

    claim_setup
        .context
        .addin
        .restrict_tokenflow(
            &claim_setup.registrar,
            &claim_setup.realm_authority,
            &claim_setup.voter,
        )
        .await?;

    claim_setup
        .context
        .addin
        .claim(
            &claim_setup.rewards_pool,
            &claim_setup.reward_mint,
            &claim_setup.deposit_mining,
            &claim_setup.voter_authority,
            &claim_setup.voter_authority_ata,
            &claim_setup.context.rewards.program_id,
            &claim_setup.registrar,
            &claim_setup.mint_governance.address,
            &claim_setup.proposal.address,
            &claim_setup.voter,
            &vote_record,
        )
        .await
        .expect_err("Claiming is restricted by Rewards program");

    claim_setup
        .context
        .addin
        .allow_tokenflow(
            &claim_setup.registrar,
            &claim_setup.realm_authority,
            &claim_setup.voter,
        )
        .await?;

    claim_setup
        .context
        .addin
        .claim(
            &claim_setup.rewards_pool,
            &claim_setup.reward_mint,
            &claim_setup.deposit_mining,
            &claim_setup.voter_authority,
            &claim_setup.voter_authority_ata,
            &claim_setup.context.rewards.program_id,
            &claim_setup.registrar,
            &claim_setup.mint_governance.address,
            &claim_setup.proposal.address,
            &claim_setup.voter,
            &vote_record,
        )
        .await?;

    let claimed_amount = claim_setup
        .context
        .solana
        .token_account_balance(claim_setup.voter_authority_ata)
        .await;
    assert_eq!(100, claimed_amount);

    Ok(())
}

#[tokio::test]
async fn claim_without_dao_vote_fail() -> Result<(), TransportError> {
    // Create realm with invalid seed
    let claim_setup = setup("Realm").await?;

    // DAO with invalid seed vote
    claim_setup
        .realm
        .cast_vote(
            claim_setup.mint_governance.address,
            &claim_setup.proposal,
            &claim_setup.voter,
            &claim_setup.voter_authority,
            &claim_setup.payer,
            claim_setup
                .context
                .addin
                .update_voter_weight_record_instruction(&claim_setup.registrar, &claim_setup.voter),
        )
        .await
        .unwrap();
    let vote_record = get_vote_record_address(
        &Pubkey::from(GOVERNANCE_PROGRAM_ID),
        &claim_setup.proposal.address,
        &claim_setup.proposal.owner_token_owner_record,
    );

    // Check do we voted into valid DAO, but we dont
    claim_setup
        .context
        .addin
        .claim(
            &claim_setup.rewards_pool,
            &claim_setup.reward_mint,
            &claim_setup.deposit_mining,
            &claim_setup.voter_authority,
            &claim_setup.voter_authority_ata,
            &claim_setup.context.rewards.program_id,
            &claim_setup.registrar,
            &claim_setup.mint_governance.address,
            &claim_setup.proposal.address,
            &claim_setup.voter,
            &vote_record,
        )
        .await
        .assert_on_chain_err(MplStakingError::NoDaoInteractionFound);

    Ok(())
}
