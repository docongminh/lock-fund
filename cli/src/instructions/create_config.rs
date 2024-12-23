use crate::*;

use anchor_client::anchor_lang::{InstructionData, ToAccountMetas};
use anchor_client::solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use anchor_lang::solana_program;
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

pub fn create_config(
    client: RpcClient,
    keypair: Keypair,
    params: CreateConfigParams,
) -> Result<Signature> {
    let (escrow, _bump) = Pubkey::find_program_address(
        &[lock_fund::ESCROW_SEED, keypair.pubkey().as_ref()],
        &lock_fund::ID,
    );
    let (config_account, _bump) =
        Pubkey::find_program_address(&[lock_fund::CONFIG_SEED, escrow.as_ref()], &lock_fund::ID);

    let ix = Instruction {
        program_id: lock_fund::ID,
        accounts: lock_fund::accounts::CreateConfig {
            authority: keypair.pubkey(),
            config_account: config_account,
            escrow: escrow,
            recipient: params.recipient,
            approver: params.approver,
            system_program: solana_program::system_program::id(),
        }
        .to_account_metas(None),
        data: lock_fund::instruction::CreateConfig {
            params: lock_fund::CreateConfigParams {
                cliff_time_duration: params.cliff_time_duration,
                amount_per_day: params.amount_per_day,
                update_actor_mode: params.update_actor_mode,
                enable_transfer_full: params.enable_transfer_full,
            },
        }
        .data(),
    };
    let blockhash = client.get_latest_blockhash().unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&keypair.pubkey()), &[&keypair], blockhash);
    let sig = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();

    Ok(sig)
}
