use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use mplx_staking_states::error::*;
use mplx_staking_states::state::*;

use crate::cpi_instructions;
use crate::cpi_instructions::REWARD_CONTRACT_ID;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter.load()?.voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = registrar)]
    pub voter: AccountLoader<'info, Voter>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = deposit_token.mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = deposit_token.owner == deposit_authority.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,
    pub deposit_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    pub rewards_program: UncheckedAccount<'info>,
}

impl<'info> Deposit<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.deposit_token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.deposit_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

/// Adds tokens to a deposit entry.
///
/// Tokens will be transfered from deposit_token to vault using the deposit_authority.
///
/// The deposit entry must have been initialized with create_deposit_entry.
///
/// `deposit_entry_index`: Index of the deposit entry.
/// `amount`: Number of native tokens to transfer.
pub fn deposit(ctx: Context<Deposit>, deposit_entry_index: u8, amount: u64) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let registrar = &ctx.accounts.registrar.load()?;
    let curr_ts = registrar.clock_unix_timestamp();

    {
        let voter = &mut ctx.accounts.voter.load_mut()?;
        let d_entry = voter.active_deposit_mut(deposit_entry_index)?;

        // Get the exchange rate entry associated with this deposit.
        let mint_idx = registrar.voting_mint_config_index(ctx.accounts.deposit_token.mint)?;
        require_eq!(
            mint_idx,
            d_entry.voting_mint_config_idx as usize,
            VsrError::InvalidMint
        );
        // Deposit tokens into the vault and increase the lockup amount too.

        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        d_entry.amount_deposited_native =
            d_entry.amount_deposited_native.checked_add(amount).unwrap();
    }

    let reward_pool = &ctx.accounts.reward_pool;
    let mining = &ctx.accounts.deposit_mining;
    let deposit_authority = &ctx.accounts.deposit_authority;
    let reward_mint = &ctx.accounts.deposit_token.mint;
    let voter = &ctx.accounts.voter.load()?;
    let d_entry = voter.active_deposit(deposit_entry_index)?;

    let (_reward_pool_pubkey, pool_bump_seed) = Pubkey::find_program_address(
        &[&reward_pool.key().to_bytes(), &reward_mint.key().to_bytes()],
        &REWARD_CONTRACT_ID,
    );

    let signers_seeds = &[
        &reward_pool.key().to_bytes()[..32],
        &reward_mint.key().to_bytes()[..32],
        &[pool_bump_seed],
    ];

    cpi_instructions::deposit_mining(
        ctx.accounts.rewards_program.to_account_info(),
        reward_pool.to_account_info(),
        mining.to_account_info(),
        ctx.accounts.voter.to_account_info(),
        deposit_authority.to_account_info(),
        amount,
        d_entry.lockup.period,
        &[signers_seeds],
        reward_mint,
    )?;

    msg!(
        "Deposited amount {} at deposit index {} with lockup kind {:?} with lockup period {:?} and {} seconds left. It's used now: {:?}",
        amount,
        deposit_entry_index,
        d_entry.lockup.kind,
        d_entry.lockup.period,
        d_entry.lockup.seconds_left(curr_ts),
        d_entry.is_used,
    );

    Ok(())
}
