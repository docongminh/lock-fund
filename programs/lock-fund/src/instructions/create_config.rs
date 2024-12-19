use anchor_lang::prelude::*;

use crate::{ConfigAccount, CreateConfigEvent, LockFundEscrowError, CONFIG_SEED, ESCROW_SEED};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateConfigParams {
    pub cliff_time_duration: u64,
    pub amount_per_day: u64,
    pub update_actor_mode: u8,
    pub enable_transfer_full: u8,
}

impl CreateConfigParams {
    pub fn validate_params(&self) -> Result<()> {
        Ok(())
    }

    pub fn init_config(
        &self,
        config_account: &AccountLoader<ConfigAccount>,
        authority: Pubkey,
        approver: Pubkey,
        recipient: Pubkey,
        escrow_vault: Pubkey,
        cliff_time_duration: u64,
        amount_per_day: u64,
        update_actor_mode: u8,
        enable_transfer_full: u8,
        escrow_bump: u8,
        escrow_vault_bump: u8,
    ) -> Result<()> {
        self.validate_params()?;

        require_keys_neq!(authority, approver, LockFundEscrowError::DuplicatePubkey);

        let mut config_account = config_account.load_init()?;
        let cliff_time = Clock::get()?.unix_timestamp as u64 + cliff_time_duration;
        config_account.init(
            authority,
            approver,
            recipient,
            escrow_vault,
            cliff_time,
            amount_per_day,
            update_actor_mode,
            enable_transfer_full,
            escrow_bump,
            escrow_vault_bump,
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            CONFIG_SEED.as_ref(),
            escrow.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + ConfigAccount::INIT_SPACE
    )]
    pub config_account: AccountLoader<'info, ConfigAccount>,

    /// CHECK: escrow vault
    #[account(
        init,
        seeds = [
            ESCROW_SEED.as_ref(),
            authority.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 0,
        owner = system_program.key()
    )]
    pub escrow: AccountInfo<'info>,

    /// CHECK: recipient account.
    pub recipient: UncheckedAccount<'info>,

    /// CHECK: recipient account.
    pub approver: UncheckedAccount<'info>,

    /// system program.
    pub system_program: Program<'info, System>,
}

pub fn create_config_handler(
    ctx: Context<CreateConfig>,
    params: &CreateConfigParams,
) -> Result<()> {
    params.init_config(
        &ctx.accounts.config_account,
        ctx.accounts.authority.key(),
        ctx.accounts.approver.key(),
        ctx.accounts.recipient.key(),
        ctx.accounts.escrow.key(),
        params.cliff_time_duration,
        params.amount_per_day,
        params.update_actor_mode,
        params.enable_transfer_full,
        ctx.bumps.config_account,
        ctx.bumps.escrow,
    )?;

    let &CreateConfigParams {
        cliff_time_duration,
        amount_per_day,
        update_actor_mode,
        enable_transfer_full,
    } = params;

    emit!(CreateConfigEvent {
        authority: ctx.accounts.authority.key(),
        approver: ctx.accounts.approver.key(),
        recipient: ctx.accounts.recipient.key(),
        cliff_time_duration,
        amount_per_day,
        update_actor_mode,
        enable_transfer_full,
    });
    Ok(())
}
