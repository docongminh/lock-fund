use anchor_lang::prelude::*;

use crate::{ConfigAccount, CreateConfigEvent, CreateConfigParams, CONFIG_SEED, ESCROW_SEED};

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
