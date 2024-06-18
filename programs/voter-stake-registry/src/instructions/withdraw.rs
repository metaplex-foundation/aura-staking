use crate::cpi_instructions::withdraw_mining;
use crate::voter::{load_token_owner_record, VoterWeightRecord};
use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub registrar: AccountLoader<'info, Registrar>,

    // checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.load()?.voter_bump,
        has_one = registrar,
        has_one = voter_authority,
    )]
    pub voter: AccountLoader<'info, Voter>,
    pub voter_authority: Signer<'info>,

    /// The token_owner_record for the voter_authority. This is needed
    /// to be able to forbid withdraws while the voter is engaged with
    /// a vote or has an open proposal.
    ///
    /// CHECK: token_owner_record is validated in the instruction:
    /// - owned by registrar.governance_program_id
    /// - for the registrar.realm
    /// - for the registrar.realm_governing_token_mint
    /// - governing_token_owner is voter_authority
    pub token_owner_record: UncheckedAccount<'info>,

    /// Withdraws must update the voter weight record, to prevent a stale
    /// record being used to vote after the withdraw.
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter_authority.key().as_ref()],
        bump = voter.load()?.voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.load()?.realm,
        constraint = voter_weight_record.governing_token_owner == voter.load()?.voter_authority,
        constraint = voter_weight_record.governing_token_mint == registrar.load()?.realm_governing_token_mint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = destination.mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination: Box<Account<'info, TokenAccount>>,

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

impl<'info> Withdraw<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.vault.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.voter.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

/// Withdraws tokens from a deposit entry, if they are unlocked
///
/// `deposit_entry_index`: The deposit entry to withdraw from.
/// `amount` is in units of the native currency being withdrawn.
pub fn withdraw(
    ctx: Context<Withdraw>,
    deposit_entry_index: u8,
    amount: u64,
    registrar_bump: u8,
    realm_governing_mint_pubkey: Pubkey,
    realm_pubkey: Pubkey,
) -> Result<()> {
    {
        // Transfer the tokens to withdraw.
        let voter = &mut ctx.accounts.voter.load()?;
        let voter_seeds = voter_seeds!(voter);
        token::transfer(
            ctx.accounts.transfer_ctx().with_signer(&[voter_seeds]),
            amount,
        )?;
    }
    {
        // Load the accounts.
        let registrar = &ctx.accounts.registrar.load()?;
        let voter = &mut ctx.accounts.voter.load_mut()?;

        // Get the exchange rate for the token being withdrawn.
        let mint_idx = registrar.voting_mint_config_index(ctx.accounts.destination.mint)?;

        // Governance may forbid withdraws, for example when engaged in a vote.
        // Not applicable for tokens that don't contribute to voting power.
        if registrar.voting_mints[mint_idx].grants_vote_weight() {
            let token_owner_record = load_token_owner_record(
                &voter.voter_authority,
                &ctx.accounts.token_owner_record.to_account_info(),
                registrar,
            )?;
            token_owner_record.assert_can_withdraw_governing_tokens()?;
        }

        // Get the deposit being withdrawn from.
        let curr_ts = registrar.clock_unix_timestamp();
        let deposit_entry = voter.active_deposit_mut(deposit_entry_index)?;

        // check whether funds are cooled down
        if deposit_entry.lockup.kind == LockupKind::Constant {
            require!(
                deposit_entry.lockup.cooldown_requested,
                VsrError::UnlockMustBeCalledFirst
            );
            require!(
                curr_ts >= deposit_entry.lockup.cooldown_ends_at,
                VsrError::InvalidTimestampArguments
            );
        }

        require_gte!(
            deposit_entry.amount_unlocked(curr_ts),
            amount,
            VsrError::InsufficientUnlockedTokens
        );
        require_eq!(
            mint_idx,
            deposit_entry.voting_mint_config_idx as usize,
            VsrError::InvalidMint
        );

        // Bookkeeping for withdrawn funds.
        require_gte!(
            deposit_entry.amount_deposited_native,
            amount,
            VsrError::InternalProgramError
        );

        deposit_entry.amount_deposited_native = deposit_entry
            .amount_deposited_native
            .checked_sub(amount)
            .unwrap();

        // if deposit doesn't have tokens after withdrawal
        // then is shouldn't be used
        if deposit_entry.amount_deposited_native == 0
            && deposit_entry.lockup.kind != LockupKind::None
            && deposit_entry.lockup.period != LockupPeriod::None
        {
            *deposit_entry = DepositEntry::default();
            deposit_entry.is_used = false;
        }

        msg!(
            "Withdrew amount {} at deposit index {} with lockup kind {:?} and {} seconds left",
            amount,
            deposit_entry_index,
            deposit_entry.lockup.kind,
            deposit_entry.lockup.seconds_left(curr_ts),
        );

        if deposit_entry.lockup.kind == LockupKind::None
            && deposit_entry.lockup.period == LockupPeriod::None
        {
            return Ok(());
        }
    }

    // Update the voter weight record
    let voter = &ctx.accounts.voter.load()?;
    let record = &mut ctx.accounts.voter_weight_record;
    record.voter_weight = voter.weight()?;
    record.voter_weight_expiry = Some(Clock::get()?.slot);

    let rewards_program = &ctx.accounts.rewards_program;
    let reward_pool = &ctx.accounts.reward_pool;
    let mining = &ctx.accounts.deposit_mining;
    let pool_deposit_authority = &ctx.accounts.registrar;
    let owner = &ctx.accounts.voter_authority;
    let signers_seeds = &[
        &realm_pubkey.key().to_bytes(),
        b"registrar".as_ref(),
        &realm_governing_mint_pubkey.key().to_bytes(),
        &[registrar_bump][..],
    ];

    withdraw_mining(
        rewards_program.to_account_info(),
        reward_pool.to_account_info(),
        mining.to_account_info(),
        pool_deposit_authority.to_account_info(),
        amount,
        owner.key,
        signers_seeds,
    )?;

    Ok(())
}
