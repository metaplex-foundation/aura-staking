use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::{Token, TokenAccount};

use crate::cpi_instructions;

#[derive(Accounts)]
pub struct Claim<'info> {
    /// CHECK: Reward Pool PDA will be checked in the rewards contract
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    pub reward_mint: Account<'info, Mint>,

    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    // pub voter_authority: Signer<'info>,
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_reward_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Rewards Program account
    pub rewards_program: UncheckedAccount<'info>,
}

/// Claims token from the Rewards Contract.
///
/// Tokens will be transfered from Vault in Rewards account to User's user_reward_token_account.
/// This call actually doesn't mutating Staking's accounts, only Reward's accounts will be mutated.
pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let rewards_program = ctx.accounts.rewards_program.to_account_info();
    let reward_pool = ctx.accounts.reward_pool.to_account_info();
    let rewards_mint = ctx.accounts.reward_mint.to_account_info();
    let vault = ctx.accounts.vault.to_account_info();
    let deposit_mining = ctx.accounts.deposit_mining.to_account_info();
    let user = ctx.accounts.owner.to_account_info();
    let user_reward_token_account = ctx.accounts.user_reward_token_account.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

    cpi_instructions::claim(
        rewards_program,
        reward_pool,
        rewards_mint,
        vault,
        deposit_mining,
        user,
        user_reward_token_account,
        token_program,
    )?;

    // TODO: add msg about claimed amount, getting use of return_data function
    Ok(())
}
