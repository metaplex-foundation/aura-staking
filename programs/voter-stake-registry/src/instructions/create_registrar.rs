use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use mplx_staking_states::error::*;
use mplx_staking_states::state::*;
use spl_governance::state::realm;
use std::mem::size_of;

use crate::cpi_instructions;

#[derive(Accounts)]
pub struct CreateRegistrar<'info> {
    /// The voting registrar. There can only be a single registrar
    /// per governance realm and governing mint.
    #[account(
        init,
        seeds = [realm.key().as_ref(), b"registrar".as_ref(), realm_governing_token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + size_of::<Registrar>()
    )]
    pub registrar: AccountLoader<'info, Registrar>,

    /// An spl-governance realm
    ///
    /// CHECK: realm is validated in the instruction:
    /// - realm is owned by the governance_program_id
    /// - realm_governing_token_mint must be the community or council mint
    /// - realm_authority is realm.authority
    pub realm: UncheckedAccount<'info>,

    /// CHECK: May be any instance of spl-governance
    /// The program id of the spl-governance program the realm belongs to.
    pub governance_program_id: UncheckedAccount<'info>,
    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,
    pub realm_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: any address is allowed
    /// Account that will be created via CPI to the rewards,
    /// it's responsible for being a "root" for all entities
    /// inside rewards contract
    #[account(mut)]
    reward_pool: UncheckedAccount<'info>,

    /// CHECK: any address is allowed
    /// This account is responsible for storing money for rewards
    #[account(mut)]
    reward_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Rewards Program account
    pub rewards_program: UncheckedAccount<'info>,
}

/// Creates a new voting registrar.
///
/// `vote_weight_decimals` is the number of decimals used on the vote weight. It must be
/// larger or equal to all token mints used for voting.
///
/// To use the registrar, call ConfigVotingMint to register token mints that may be
/// used for voting.
pub fn create_registrar(
    ctx: Context<CreateRegistrar>,
    registrar_bump: u8,
    fill_authority: Pubkey,
    distribution_authority: Pubkey,
) -> Result<()> {
    {
        let registrar = &mut ctx.accounts.registrar.load_init()?;
        require_eq!(registrar_bump, *ctx.bumps.get("registrar").unwrap());
        registrar.bump = registrar_bump;
        registrar.governance_program_id = ctx.accounts.governance_program_id.key();
        registrar.realm = ctx.accounts.realm.key();
        registrar.realm_governing_token_mint = ctx.accounts.realm_governing_token_mint.key();
        registrar.realm_authority = ctx.accounts.realm_authority.key();
        registrar.time_offset = 0;

        // Verify that "realm_authority" is the expected authority on "realm"
        // and that the mint matches one of the realm mints too.
        let realm = realm::get_realm_data_for_governing_token_mint(
            &registrar.governance_program_id,
            &ctx.accounts.realm.to_account_info(),
            &registrar.realm_governing_token_mint,
        )?;
        require_keys_eq!(
            realm.authority.unwrap(),
            ctx.accounts.realm_authority.key(),
            VsrError::InvalidRealmAuthority
        );
    }

    // we should initiate the rewards pool to proceed with
    // staking and rewards logic
    let rewards_program_id = ctx.accounts.rewards_program.to_account_info();
    let reward_pool = ctx.accounts.reward_pool.to_account_info();
    let reward_mint = ctx.accounts.realm_governing_token_mint.to_account_info();
    let reward_vault = ctx.accounts.reward_vault.to_account_info();
    let payer = ctx.accounts.payer.to_account_info();
    let rent = ctx.accounts.rent.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();
    let deposit_authority = ctx.accounts.registrar.key();

    cpi_instructions::initialize_pool(
        rewards_program_id,
        reward_pool,
        reward_mint,
        reward_vault,
        payer,
        rent,
        token_program,
        system_program,
        deposit_authority,
        fill_authority,
        distribution_authority,
    )?;

    Ok(())
}
