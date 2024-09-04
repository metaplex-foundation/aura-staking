use crate::{
    clock_unix_timestamp, cpi_instructions,
    voter::{load_token_owner_record, VoterWeightRecord},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use mplx_staking_states::{
    error::MplStakingError,
    registrar_seeds,
    state::{DepositEntry, Registrar, Voter},
    voter_seeds,
};

#[derive(Accounts)]
pub struct Slashing<'info> {
    /// The voting registrar. There can only be a single registrar
    /// per governance realm and governing mint.
    #[account(
        seeds = [realm.key().as_ref(), b"registrar".as_ref(), realm_treasury.mint.key().as_ref()],
        bump,
    )]
    pub registrar: AccountLoader<'info, Registrar>,

    #[account(mut)]
    pub voter: AccountLoader<'info, Voter>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = realm_treasury.mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: it's only needed to verify registrar
    pub realm: UncheckedAccount<'info>,

    /// CHECK:
    pub token_owner_record: UncheckedAccount<'info>,

    #[account(mut)]
    pub realm_treasury: Box<Account<'info, TokenAccount>>,

    pub realm_authority: Signer<'info>,

    /// Withdraws must update the voter weight record, to prevent a stale
    /// record being used to vote after the withdraw.
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter.load()?.voter_authority.key().as_ref()],
        bump = voter.load()?.voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.load()?.realm,
        constraint = voter_weight_record.governing_token_owner == voter.load()?.voter_authority,
        constraint = voter_weight_record.governing_token_mint == registrar.load()?.realm_governing_token_mint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// CHECK:
    /// Ownership of the account will be checked in the rewards contract
    /// It's the core account for the rewards contract, which will
    /// keep track of all rewards and staking logic.
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,

    /// CHECK: mining PDA will be checked in the rewards contract
    /// PDA(["mining", mining owner <aka voter_authority in our case>, reward_pool],
    /// reward_program)
    #[account(mut)]
    pub deposit_mining: UncheckedAccount<'info>,

    /// CHECK: Rewards Program account
    #[account(executable)]
    pub rewards_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Slashing<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.vault.to_account_info(),
            to: self.realm_treasury.to_account_info(),
            authority: self.voter.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

/// Slashes the specified stake in transfers money to the treasury.
///
/// `deposit_entry_index`: The deposit entry to withdraw from.
/// `amount` is in units of the native currency being withdrawn.
pub fn slash(
    ctx: Context<Slashing>,
    deposit_entry_index: u8,
    amount: u64,
    mining_owner: Pubkey,
) -> Result<()> {
    let registrar = ctx.accounts.registrar.load()?;

    // this block is needed not to violate borrowing rules for the voter
    {
        require_keys_eq!(
            registrar.realm_authority,
            ctx.accounts.realm_authority.key(),
            MplStakingError::InvalidRealmAuthority
        );

        let voter = &mut ctx.accounts.voter.load_mut()?;
        // Governance may forbid withdraws, for example when engaged in a vote.
        // Not applicable for tokens that don't contribute to voting power.
        let token_owner_record = load_token_owner_record(
            &voter.voter_authority,
            &ctx.accounts.token_owner_record.to_account_info(),
            &registrar,
        )?;
        token_owner_record.assert_can_withdraw_governing_tokens()?;

        let deposit_entry = voter.active_deposit_mut(deposit_entry_index)?;

        let mint_idx = registrar.voting_mint_config_index(ctx.accounts.realm_treasury.mint)?;
        require_eq!(
            mint_idx,
            deposit_entry.voting_mint_config_idx as usize,
            MplStakingError::InvalidMint
        );

        // Bookkeeping for withdrawn funds.
        require_gte!(
            deposit_entry.amount_deposited_native,
            amount,
            MplStakingError::InternalProgramError
        );

        deposit_entry.amount_deposited_native = deposit_entry
            .amount_deposited_native
            .checked_sub(amount)
            .ok_or(MplStakingError::ArithmeticOverflow)?;

        // if deposit doesn't have tokens after withdrawal
        // then is shouldn't be used
        if deposit_entry.amount_deposited_native == 0 {
            *deposit_entry = DepositEntry::default();
            deposit_entry.is_used = false;
        }

        msg!(
            "Slashed amount {} at deposit index {} with lockup kind {:?}",
            amount,
            deposit_entry_index,
            deposit_entry.lockup.kind,
        );

        let slash_amount_multiplied_by_period = amount
            .checked_mul(deposit_entry.lockup.period.multiplier())
            .ok_or(MplStakingError::ArithmeticOverflow)?;
        let amount = amount
            .checked_mul(deposit_entry.lockup.period.multiplier())
            .ok_or(MplStakingError::ArithmeticOverflow)?;
        let curr_ts = clock_unix_timestamp();
        let stake_expiration_date = if curr_ts > deposit_entry.lockup.end_ts {
            None
        } else {
            Some(deposit_entry.lockup.end_ts)
        };
        let signers_seeds = registrar_seeds!(&registrar);

        cpi_instructions::slash(
            ctx.accounts.rewards_program.to_account_info(),
            ctx.accounts.registrar.to_account_info(),
            ctx.accounts.reward_pool.to_account_info(),
            ctx.accounts.deposit_mining.to_account_info(),
            &mining_owner,
            amount,
            slash_amount_multiplied_by_period,
            stake_expiration_date,
            signers_seeds,
        )?;

        // Update the voter weight record
        let record = &mut ctx.accounts.voter_weight_record;
        record.voter_weight = voter.weight()?;
        record.voter_weight_expiry = Some(Clock::get()?.slot);
    }

    let voter = ctx.accounts.voter.load()?;
    let voter_seeds = voter_seeds!(voter);
    token::transfer(
        ctx.accounts.transfer_ctx().with_signer(&[voter_seeds]),
        amount,
    )?;

    Ok(())
}
