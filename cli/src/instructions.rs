use std::rc::Rc;
use std::str::FromStr;

use anchor_client::anchor_lang::solana_program;
use anchor_client::solana_sdk::program_pack::Pack;
use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Signature},
    signer::{keypair::Keypair, Signer},
};
use anchor_client::Client;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::{spl_token::state::Mint, ID};
use anyhow::{Ok, Result};

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
    pub escrow: Pubkey,
    pub config_account: Pubkey,
}

impl LockFundProgram {
    pub fn config_data(&self, config_account: Option<String>) -> Result<lock_fund::ConfigAccount> {
        let mut account = self.escrow;
        if let Some(config_account) = config_account {
            account = Pubkey::from_str(&config_account)?;
        }

        let data: lock_fund::ConfigAccount = self.program.account(account)?;
        Ok(data)
    }

    pub fn init(params: InitProgramParams) -> Self {
        let InitProgramParams {
            rpc_url,
            wss_url,
            approver_path,
            authority_path,
        } = params;
        let authority = read_keypair_file(authority_path).unwrap();
        let authority_pubkey = authority.pubkey();
        let approver = read_keypair_file(approver_path).unwrap();
        let anchor_client = Client::new(
            anchor_client::Cluster::Custom(rpc_url, wss_url),
            Rc::new(authority),
        );
        let program = anchor_client.program(lock_fund::ID).unwrap();
        let (escrow, _bump) = Pubkey::find_program_address(
            &[lock_fund::ESCROW_SEED, authority_pubkey.as_ref()],
            &lock_fund::ID,
        );
        let (config_account, _bump) = Pubkey::find_program_address(
            &[lock_fund::CONFIG_SEED, escrow.as_ref()],
            &lock_fund::ID,
        );
        LockFundProgram {
            program,
            approver,
            escrow,
            config_account,
        }
    }
    pub fn create_config(&self, params: CreateConfigParams) -> Result<Signature> {
        let sig: Signature = self
            .program
            .request()
            .accounts(lock_fund::accounts::CreateConfig {
                authority: self.program.payer(),
                config_account: self.config_account,
                escrow: self.escrow,
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

    pub fn transfer_token(&self, mint: Pubkey, amount: f64) -> Result<Signature> {
        let config_account_data: lock_fund::ConfigAccount =
            self.program.account(self.config_account)?;
        let escrow_token = get_associated_token_address(&self.escrow, &mint);
        let recipient_token = get_associated_token_address(&config_account_data.recipient, &mint);
        let mint_account = self.program.rpc().get_account(&mint).unwrap();
        let decimals = Mint::unpack(&mint_account.data).unwrap().decimals;
        let raw_amount = amount * 10u64.pow(decimals as u32) as f64;

        //
        let recipient_token_data = self.program.rpc().get_token_account(&recipient_token)?;

        let (event_authority, _bump) =
            Pubkey::find_program_address(&[b"__event_authority"], &lock_fund::ID);

        let sig = self
            .program
            .request()
            .accounts(lock_fund::accounts::TransferToken {
                config_account: self.config_account,
                escrow: self.escrow,
                escrow_token,
                recipient_token,
                recipient: config_account_data.recipient,
                mint_token: mint,
                authority: self.program.payer(),
                approver: self.approver.pubkey(),
                token_program: ID,
                event_authority,
                program: lock_fund::ID,
            })
            .args(lock_fund::instruction::TransferToken { amount: raw_amount as u64 })
            .signer(&self.approver)
            .send()?;
        Ok(sig)
    }

    pub fn transfer_sol(&self, amount: u64) -> Result<Signature> {
        let config_account_data: lock_fund::ConfigAccount =
            self.program.account(self.config_account)?;

        let (event_authority, _bump) =
            Pubkey::find_program_address(&[b"__event_authority"], &lock_fund::ID);
        let sig = self
            .program
            .request()
            .accounts(lock_fund::accounts::TransferSol {
                config_account: self.config_account,
                escrow: self.escrow,
                recipient: config_account_data.recipient,
                authority: self.program.payer(),
                approver: self.approver.pubkey(),
                event_authority,
                system_program: solana_program::system_program::id(),
                program: lock_fund::ID,
            })
            .args(lock_fund::instruction::TransferSol { amount })
            .signer(&self.approver)
            .send()?;
        Ok(sig)
    }
}
