use crate::*;
use anchor_lang::Key;
use mplx_staking_states::state::Voter;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct AddinCookie {
    pub solana: Rc<solana::SolanaCookie>,
    pub program_id: Pubkey,
    pub time_offset: RefCell<i64>,
}

pub struct RegistrarCookie {
    pub address: Pubkey,
    pub authority: Pubkey,
    pub mint: MintCookie,
    pub registrar_bump: u8,
    pub realm_pubkey: Pubkey,
    pub realm_governing_token_mint_pubkey: Pubkey,
}

#[derive(Clone)]
pub struct VotingMintConfigCookie {
    pub mint: MintCookie,
}

impl VotingMintConfigCookie {
    #[allow(dead_code)]
    pub async fn vault_balance(&self, solana: &SolanaCookie, voter: &VoterCookie) -> u64 {
        let vault = voter.vault_address(self);
        solana.get_account::<TokenAccount>(vault).await.amount
    }
}

pub struct VoterCookie {
    pub address: Pubkey,
    pub authority: Pubkey,
    pub voter_weight_record: Pubkey,
    pub token_owner_record: Pubkey,
}

impl AddinCookie {
    pub async fn create_registrar(
        &self,
        realm: &GovernanceRealmCookie,
        authority: &Keypair,
        payer: &Keypair,
        fill_authority: &Pubkey,
        distribution_authority: &Pubkey,
        rewards_program: &Pubkey,
    ) -> (RegistrarCookie, Pubkey) {
        let community_token_mint = realm.community_token_mint.pubkey.unwrap();

        let (registrar, registrar_bump) = Pubkey::find_program_address(
            &[
                &realm.realm.to_bytes(),
                b"registrar".as_ref(),
                &community_token_mint.to_bytes(),
            ],
            &self.program_id,
        );

        let data = anchor_lang::InstructionData::data(
            &voter_stake_registry::instruction::CreateRegistrar {
                registrar_bump,
                fill_authority: *fill_authority,
                distribution_authority: *distribution_authority,
            },
        );

        let (reward_pool, _reward_pool_bump) = Pubkey::find_program_address(
            &["reward_pool".as_bytes(), &registrar.key().to_bytes()],
            rewards_program,
        );

        let (reward_vault, _reward_vault_bump) = Pubkey::find_program_address(
            &[
                "vault".as_bytes(),
                &reward_pool.key().to_bytes(),
                &community_token_mint.to_bytes(),
            ],
            rewards_program,
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::CreateRegistrar {
                registrar,
                governance_program_id: realm.governance.program_id,
                realm: realm.realm,
                realm_governing_token_mint: community_token_mint,
                realm_authority: realm.authority,
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                rent: solana_program::sysvar::rent::id(),
                reward_pool,
                reward_vault,
                token_program: spl_token::id(),
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[payer, authority]))
            .await
            .unwrap();

        let registrar_cookie = RegistrarCookie {
            address: registrar,
            authority: realm.authority,
            mint: realm.community_token_mint.clone(),
            registrar_bump,
            realm_pubkey: realm.realm,
            realm_governing_token_mint_pubkey: community_token_mint,
        };

        (registrar_cookie, reward_pool)
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn stake(
        &self,
        // accounts
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voter_authority: &Keypair,
        rewards_program: &Pubkey,
        // params
        source_deposit_entry_index: u8,
        target_deposit_entry_index: u8,
        amount: u64,
    ) -> std::result::Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(&voter_stake_registry::instruction::Stake {
            source_deposit_entry_index,
            target_deposit_entry_index,
            amount,
        });

        let (reward_pool, _reward_pool_bump) = Pubkey::find_program_address(
            &["reward_pool".as_bytes(), &registrar.address.to_bytes()],
            rewards_program,
        );

        let deposit_mining =
            find_deposit_mining_addr(&voter_authority.pubkey(), &reward_pool, rewards_program);

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::Stake {
                registrar: registrar.address,
                voter: voter.address,
                voter_authority: voter_authority.pubkey(),
                reward_pool,
                deposit_mining,
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[voter_authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn configure_voting_mint(
        &self,
        registrar: &RegistrarCookie,
        authority: &Keypair,
        _payer: &Keypair,
        index: u16,
        mint: &MintCookie,
        grant_authority: Option<Pubkey>,
        other_mints: Option<&[Pubkey]>,
    ) -> VotingMintConfigCookie {
        let deposit_mint = mint.pubkey.unwrap();

        let data = anchor_lang::InstructionData::data(
            &voter_stake_registry::instruction::ConfigureVotingMint {
                idx: index,
                grant_authority,
            },
        );

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::ConfigureVotingMint {
                mint: deposit_mint,
                registrar: registrar.address,
                realm_authority: authority.pubkey(),
            },
            None,
        );
        accounts.push(anchor_lang::prelude::AccountMeta::new_readonly(
            deposit_mint,
            false,
        ));
        for mint in other_mints.unwrap_or(&[]) {
            accounts.push(anchor_lang::prelude::AccountMeta::new_readonly(
                *mint, false,
            ));
        }

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[authority]))
            .await
            .unwrap();

        VotingMintConfigCookie { mint: mint.clone() }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_voter(
        &self,
        registrar: &RegistrarCookie,
        token_owner_record: &TokenOwnerRecordCookie,
        authority: &Keypair,
        payer: &Keypair,
        reward_pool: &Pubkey,
        deposit_mining: &Pubkey,
        rewards_program: &Pubkey,
    ) -> VoterCookie {
        let (voter, voter_bump) = Pubkey::find_program_address(
            &[
                &registrar.address.to_bytes(),
                b"voter".as_ref(),
                &authority.pubkey().to_bytes(),
            ],
            &self.program_id,
        );
        let (voter_weight_record, voter_weight_record_bump) = Pubkey::find_program_address(
            &[
                &registrar.address.to_bytes(),
                b"voter-weight-record".as_ref(),
                &authority.pubkey().to_bytes(),
            ],
            &self.program_id,
        );

        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::CreateVoter {
                voter_bump,
                voter_weight_record_bump,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::CreateVoter {
                voter,
                voter_weight_record,
                registrar: registrar.address,
                voter_authority: authority.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                rent: solana_program::sysvar::rent::id(),
                instructions: solana_program::sysvar::instructions::id(),
                reward_pool: *reward_pool,
                deposit_mining: *deposit_mining,
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[payer, authority]))
            .await
            .unwrap();

        VoterCookie {
            address: voter,
            authority: authority.pubkey(),
            voter_weight_record,
            token_owner_record: token_owner_record.address,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_deposit_entry(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voter_authority: &Keypair,
        voting_mint: &VotingMintConfigCookie,
        deposit_entry_index: u8,
        lockup_kind: LockupKind,
        period: LockupPeriod,
        delegate: Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let vault = voter.vault_address(voting_mint);

        let data = anchor_lang::InstructionData::data(
            &voter_stake_registry::instruction::CreateDepositEntry {
                deposit_entry_index,
                kind: lockup_kind,
                period,
                delegate,
            },
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::CreateDepositEntry {
                vault,
                registrar: registrar.address,
                voter: voter.address,
                voter_authority: voter_authority.pubkey(),
                payer: voter_authority.pubkey(),
                deposit_mint: voting_mint.mint.pubkey.unwrap(),
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
                associated_token_program: spl_associated_token_account::id(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[voter_authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn deposit(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voting_mint: &VotingMintConfigCookie,
        deposit_authority: &Keypair,
        token_address: Pubkey,
        deposit_entry_index: u8,
        amount: u64,
    ) -> std::result::Result<(), BanksClientError> {
        let vault = voter.vault_address(voting_mint);

        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::Deposit {
                deposit_entry_index,
                amount,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::Deposit {
                registrar: registrar.address,
                voter: voter.address,
                vault,
                deposit_token: token_address,
                deposit_authority: deposit_authority.pubkey(),
                token_program: spl_token::id(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[deposit_authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn extend_deposit(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voting_mint: &VotingMintConfigCookie,
        deposit_authority: &Keypair,
        token_address: Pubkey,
        deposit_entry_index: u8,
        new_lockup_period: LockupPeriod,
        additional_amount: u64,
        reward_pool: &Pubkey,
        deposit_mining: &Pubkey,
        rewards_program: &Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let vault = voter.vault_address(voting_mint);

        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::ExtendStake {
                deposit_entry_index,
                new_lockup_period,
                registrar_bump: registrar.registrar_bump,
                realm_governing_mint_pubkey: registrar.realm_governing_token_mint_pubkey,
                realm_pubkey: registrar.realm_pubkey,
                additional_amount,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::ExtendStake {
                registrar: registrar.address,
                voter: voter.address,
                vault,
                deposit_token: token_address,
                deposit_authority: deposit_authority.pubkey(),
                token_program: spl_token::id(),
                reward_pool: *reward_pool,
                deposit_mining: *deposit_mining,
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[deposit_authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn unlock_tokens(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        authority: &Keypair,
        deposit_entry_index: u8,
        reward_pool: &Pubkey,
        deposit_mining: &Pubkey,
        rewards_program: &Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::UnlockTokens {
                deposit_entry_index,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::UnlockTokens {
                registrar: registrar.address,
                voter: voter.address,
                voter_authority: authority.pubkey(),
                reward_pool: *reward_pool,
                deposit_mining: *deposit_mining,
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn withdraw(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voting_mint: &VotingMintConfigCookie,
        authority: &Keypair,
        token_address: Pubkey,
        deposit_entry_index: u8,
        amount: u64,
    ) -> std::result::Result<(), BanksClientError> {
        let vault = voter.vault_address(voting_mint);

        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::Withdraw {
                deposit_entry_index,
                amount,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::Withdraw {
                registrar: registrar.address,
                voter: voter.address,
                token_owner_record: voter.token_owner_record,
                voter_weight_record: voter.voter_weight_record,
                vault,
                destination: token_address,
                voter_authority: authority.pubkey(),
                token_program: spl_token::id(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn close_voter(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        voting_mint: &VotingMintConfigCookie,
        voter_authority: &Keypair,
        rewards_program: &Pubkey,
    ) -> std::result::Result<(), BanksClientError> {
        let vault = voter.vault_address(voting_mint);

        let (reward_pool, _reward_pool_bump) = Pubkey::find_program_address(
            &["reward_pool".as_bytes(), &registrar.address.to_bytes()],
            rewards_program,
        );

        let deposit_mining =
            find_deposit_mining_addr(&voter_authority.pubkey(), &reward_pool, rewards_program);

        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::CloseVoter {});

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::CloseVoter {
                registrar: registrar.address,
                voter: voter.address,
                voter_authority: voter_authority.pubkey(),
                deposit_mining,
                reward_pool,
                sol_destination: voter_authority.pubkey(),
                token_program: spl_token::id(),
                rewards_program: *rewards_program,
            },
            None,
        );
        accounts.push(anchor_lang::prelude::AccountMeta::new(vault, false));

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[voter_authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub fn update_voter_weight_record_instruction(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
    ) -> Instruction {
        let data = anchor_lang::InstructionData::data(
            &voter_stake_registry::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::UpdateVoterWeightRecord {
                registrar: registrar.address,
                voter: voter.address,
                voter_weight_record: voter.voter_weight_record,
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
    ) -> std::result::Result<voter_stake_registry::voter::VoterWeightRecord, BanksClientError> {
        let instructions = vec![self.update_voter_weight_record_instruction(registrar, voter)];

        self.solana.process_transaction(&instructions, None).await?;

        Ok(self
            .solana
            .get_account::<voter_stake_registry::voter::VoterWeightRecord>(
                voter.voter_weight_record,
            )
            .await)
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn close_deposit_entry(
        &self,
        voter: &VoterCookie,
        authority: &Keypair,
        deposit_entry_index: u8,
    ) -> std::result::Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &voter_stake_registry::instruction::CloseDepositEntry {
                deposit_entry_index,
            },
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::CloseDepositEntry {
                voter: voter.address,
                voter_authority: authority.pubkey(),
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[authority]))
            .await
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn log_voter_info(
        &self,
        registrar: &RegistrarCookie,
        voter: &VoterCookie,
        deposit_entry_begin: u8,
    ) {
        let data =
            anchor_lang::InstructionData::data(&voter_stake_registry::instruction::LogVoterInfo {
                deposit_entry_begin,
                deposit_entry_count: 8,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::LogVoterInfo {
                registrar: registrar.address,
                voter: voter.address,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, None)
            .await
            .unwrap();
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn set_time_offset(
        &self,
        _registrar: &RegistrarCookie,
        _authority: &Keypair,
        time_offset: i64,
    ) {
        let old_offset = *self.time_offset.borrow();
        *self.time_offset.borrow_mut() = time_offset;

        let old_clock = self
            .solana
            .context
            .borrow_mut()
            .banks_client
            .get_sysvar::<solana_program::clock::Clock>()
            .await
            .unwrap();

        let mut new_clock = old_clock;
        new_clock.unix_timestamp += time_offset - old_offset;
        self.solana.context.borrow_mut().set_sysvar(&new_clock);
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub async fn claim(
        &self,
        reward_pool: &Pubkey,
        reward_mint: &Pubkey,
        reward_mining: &Pubkey,
        mining_owner: &Keypair,
        owner_reward_ata: &Pubkey,
        rewards_program: &Pubkey,
        registrar: &RegistrarCookie,
    ) -> std::result::Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(&voter_stake_registry::instruction::Claim {
            realm_governing_mint_pubkey: registrar.realm_governing_token_mint_pubkey,
            registrar_bump: registrar.registrar_bump,
            realm_pubkey: registrar.realm_pubkey,
        });

        let (vault_pubkey, _) = Pubkey::find_program_address(
            &[
                b"vault".as_ref(),
                reward_pool.as_ref(),
                reward_mint.as_ref(),
            ],
            rewards_program,
        );

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &voter_stake_registry::accounts::Claim {
                reward_pool: *reward_pool,
                reward_mint: *reward_mint,
                vault: vault_pubkey,
                deposit_mining: *reward_mining,
                mining_owner: mining_owner.pubkey(),
                registrar: registrar.address,
                user_reward_token_account: *owner_reward_ata,
                token_program: spl_token::id(),
                rewards_program: *rewards_program,
            },
            None,
        );

        let instructions = vec![Instruction {
            program_id: self.program_id,
            accounts,
            data,
        }];

        self.solana
            .process_transaction(&instructions, Some(&[mining_owner]))
            .await
    }
}

impl VoterCookie {
    #[allow(dead_code)]
    pub async fn deposit_amount(&self, solana: &SolanaCookie, deposit_id: u8) -> u64 {
        solana.get_account::<Voter>(self.address).await.deposits[deposit_id as usize]
            .amount_deposited_native
    }

    pub fn vault_address(&self, mint: &VotingMintConfigCookie) -> Pubkey {
        spl_associated_token_account::get_associated_token_address(
            &self.address,
            &mint.mint.pubkey.unwrap(),
        )
    }
}
