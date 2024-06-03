use std::borrow::BorrowMut;

use bytemuck::{bytes_of, Contiguous};
use solana_program::program_error::ProgramError;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::transport::TransportError;

use crate::TestContext;

#[allow(dead_code)]
pub fn gen_signer_seeds<'a>(nonce: &'a u64, acc_pk: &'a Pubkey) -> [&'a [u8]; 2] {
    [acc_pk.as_ref(), bytes_of(nonce)]
}

#[allow(dead_code)]
pub fn gen_signer_key(
    nonce: u64,
    acc_pk: &Pubkey,
    program_id: &Pubkey,
) -> Result<Pubkey, ProgramError> {
    let seeds = gen_signer_seeds(&nonce, acc_pk);
    Ok(Pubkey::create_program_address(&seeds, program_id)?)
}

#[allow(dead_code)]
pub fn create_signer_key_and_nonce(program_id: &Pubkey, acc_pk: &Pubkey) -> (Pubkey, u64) {
    for i in 0..=u64::MAX_VALUE {
        if let Ok(pk) = gen_signer_key(i, acc_pk, program_id) {
            return (pk, i);
        }
    }
    panic!("Could not generate signer key");
}

#[allow(dead_code)]
pub fn clone_keypair(keypair: &Keypair) -> Keypair {
    Keypair::from_base58_string(&keypair.to_base58_string())
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    mint: &Keypair,
    manager: &Pubkey,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                manager,
                None,
                0,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn initialize_rewards_contract(
    payer: &Keypair,
    context: &TestContext,
    reward_mint: &Pubkey,
    deposit_authority: &Pubkey,
) -> Result<Pubkey, TransportError> {
    let rewards_root = context.rewards.initialize_root(payer).await?;
    let rewards_pool = context
        .rewards
        .initialize_pool(&rewards_root.pubkey(), deposit_authority, payer)
        .await?;
    let _vault = context
        .rewards
        .add_vault(&rewards_root.pubkey(), &rewards_pool, reward_mint, payer)
        .await?;

    Ok(rewards_pool)
}

pub fn find_deposit_mining_addr(
    user: &Pubkey,
    rewards_pool: &Pubkey,
    rewards_program_addr: &Pubkey,
) -> Pubkey {
    let (deposit_mining, _bump) = Pubkey::find_program_address(
        &[
            "mining".as_bytes(),
            &user.to_bytes(),
            &rewards_pool.to_bytes(),
        ],
        rewards_program_addr,
    );
    deposit_mining
}

pub async fn advance_clock_by_ts(context: &mut ProgramTestContext, ts: i64) {
    let old_clock = context
        .banks_client
        .get_sysvar::<solana_program::clock::Clock>()
        .await
        .unwrap();

    let initial_slot = context.banks_client.get_root_slot().await.unwrap();
    context
        .warp_to_slot(initial_slot + (ts / 2) as u64)
        .unwrap();

    let mut new_clock = old_clock.clone();
    new_clock.unix_timestamp += ts;
    context.borrow_mut().set_sysvar(&new_clock);
}
