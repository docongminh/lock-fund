use std::rc::Rc;

use anchor_client::anchor_lang::{solana_program, InstructionData, ToAccountMetas};
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use anchor_client::Client;
use anchor_spl::associated_token::{get_associated_token_address, spl_associated_token_account};
use anchor_spl::token;
use anyhow::{Ok, Result};
use solana_rpc_client::rpc_client::RpcClient;

pub struct CreateConfigParams {
    pub cliff_time_duration: u64,
    pub amount_per_day: u64,
    pub update_actor_mode: u8,
    pub enable_transfer_full: u8,
    pub recipient: Pubkey,
    pub approver: Pubkey,
}

#[derive(Debug, Clone)]
pub struct InitProgramParams {
    pub rpc_url: String,
    pub wss_url: String,
    pub authority_path: String,
    pub approver_path: String,
}

pub struct LockFundProgram {
    pub program: anchor_client::Program<Rc<Keypair>>,
    pub approver: Keypair,
}

impl LockFundProgram {
    pub fn init(&self, params: InitProgramParams) -> Self {
        let InitProgramParams {
            rpc_url,
            wss_url,
            approver_path,
            authority_path,
        } = params;
        let authority = read_keypair_file(authority_path).unwrap();
        let approver = read_keypair_file(approver_path).unwrap();
        let anchor_client = Client::new(
            anchor_client::Cluster::Custom(rpc_url, wss_url),
            Rc::new(authority),
        );
        let program = anchor_client.program(lock_fund::ID).unwrap();

        LockFundProgram { program, approver }
    }
    pub fn create_config(&self, keypair: Keypair, params: CreateConfigParams) -> Result<Signature> {
        let (escrow, _bump) = Pubkey::find_program_address(
            &[lock_fund::ESCROW_SEED, keypair.pubkey().as_ref()],
            &lock_fund::ID,
        );
        let (config_account, _bump) = Pubkey::find_program_address(
            &[lock_fund::CONFIG_SEED, escrow.as_ref()],
            &lock_fund::ID,
        );
        let sig = self
            .program
            .request()
            .accounts(lock_fund::accounts::CreateConfig {
                authority: keypair.pubkey(),
                config_account,
                escrow,
                recipient: params.recipient,
                approver: params.approver,
                system_program: solana_program::system_program::id(),
            })
            .args(lock_fund::instruction::CreateConfig {
                params: lock_fund::CreateConfigParams {
                    cliff_time_duration: params.cliff_time_duration,
                    amount_per_day: params.amount_per_day,
                    update_actor_mode: params.update_actor_mode,
                    enable_transfer_full: params.enable_transfer_full,
                },
            })
            .send()?;

        Ok(sig)
    }

    pub fn transfer_token(
        &self,
        keypairs: [Keypair; 2],
        token: Pubkey,
        amount: u64,
    ) -> Result<Signature> {
        let (escrow, _bump) = Pubkey::find_program_address(
            &[lock_fund::ESCROW_SEED, keypairs[0].pubkey().as_ref()],
            &lock_fund::ID,
        );
        let (config_account, _bump) = Pubkey::find_program_address(
            &[lock_fund::CONFIG_SEED, escrow.as_ref()],
            &lock_fund::ID,
        );
        let config_account_data: lock_fund::ConfigAccount = self.program.account(config_account)?;
        let recipient_token = get_associated_token_address(&config_account_data.recipient, &token);
        let recipient_token_data = self.program.rpc().get_token_account(&recipient_token)?;

        let (event_authority, _bump) =
            Pubkey::find_program_address(&[b"__event_authority"], &lock_fund::ID);

        let sig = self
            .program
            .request()
            .accounts(lock_fund::accounts::TransferToken {
                config_account,
                escrow,
                escrow_token: escrow,
                recipient_token,
                recipient: config_account_data.recipient,
                mint_token: token,
                authority: self.program.payer(),
                approver: self.approver.pubkey(),
                token_program: token::ID,
                event_authority,
                program: lock_fund::ID,
            })
            .args(lock_fund::instruction::TransferToken { amount })
            .send()?;
        Ok(sig)
    }

    pub fn transfer_sol(&self, keypairs: [Keypair; 2], amount: u64) -> Result<Signature> {
        let (escrow, _bump) = Pubkey::find_program_address(
            &[lock_fund::ESCROW_SEED, keypairs[0].pubkey().as_ref()],
            &lock_fund::ID,
        );
        let (config_account, _bump) = Pubkey::find_program_address(
            &[lock_fund::CONFIG_SEED, escrow.as_ref()],
            &lock_fund::ID,
        );
        let config_account_data: lock_fund::ConfigAccount = self.program.account(config_account)?;

        let (event_authority, _bump) =
            Pubkey::find_program_address(&[b"__event_authority"], &lock_fund::ID);

        let sig = self
            .program
            .request()
            .accounts(lock_fund::accounts::TransferSol {
                config_account,
                escrow,
                recipient: config_account_data.recipient,
                authority: self.program.payer(),
                approver: self.approver.pubkey(),
                event_authority,
                system_program: solana_program::system_program::id(),
                program: lock_fund::ID,
            })
            .args(lock_fund::instruction::TransferToken { amount })
            .send()?;
        Ok(sig)
    }
}
