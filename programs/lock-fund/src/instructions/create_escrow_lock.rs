use anchor_lang::prelude::*;

use crate::{CreateLockFundEscrowEvent, LockFundEscrow, LockFundEscrowError, ESCROW_SEED};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateEscrowFundParams {
    pub cliff_time: u64,
    pub amount_per_day: u64,
    pub escrow_bump: u8,
    pub update_actor_mode: u8,
    pub enable_withdrawl_full: u8,
}

impl CreateEscrowFundParams {
    pub fn validate_params(&self) -> Result<()> {
        Ok(())
    }

    pub fn init_escrow_lock(
        &self,
        lock_fund_escrow: &AccountLoader<LockFundEscrow>,
        authority: Pubkey,
        approver: Pubkey,
        recipient: Pubkey,
        cliff_time: u64,
        amount_per_day: u64,
        escrow_bump: u8,
        update_actor_mode: u8,
        enable_withdrawl_full: u8,
    ) -> Result<()> {
        self.validate_params()?;

        require_keys_neq!(authority, approver, LockFundEscrowError::DuplicatePubkey);

        let mut lock_fund_escrow = lock_fund_escrow.load_init()?;
        lock_fund_escrow.init(
            authority,
            approver,
            recipient,
            cliff_time,
            amount_per_day,
            escrow_bump,
            update_actor_mode,
            enable_withdrawl_full,
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLockFundEscrow<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            ESCROW_SEED.as_ref(),
            authority.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + LockFundEscrow::INIT_SPACE
    )]
    pub lock_fund_escrow: AccountLoader<'info, LockFundEscrow>,

    /// CHECK: recipient account.
    pub recipient: UncheckedAccount<'info>,

    /// CHECK: recipient account.
    pub approver: UncheckedAccount<'info>,

    /// system program.
    pub system_program: Program<'info, System>,
}

pub fn create_lock_fund_escrow_handler(
    ctx: Context<CreateLockFundEscrow>,
    params: &CreateEscrowFundParams,
) -> Result<()> {
    params.init_escrow_lock(
        &ctx.accounts.lock_fund_escrow,
        ctx.accounts.authority.key(),
        ctx.accounts.approver.key(),
        ctx.accounts.recipient.key(),
        params.cliff_time,
        params.amount_per_day,
        ctx.bumps.lock_fund_escrow,
        params.update_actor_mode,
        params.enable_withdrawl_full,
    )?;

    let &CreateEscrowFundParams {
        cliff_time,
        amount_per_day,
        escrow_bump,
        update_actor_mode,
        enable_withdrawl_full,
    } = params;

    emit!(CreateLockFundEscrowEvent {
        authority: ctx.accounts.authority.key(),
        approver: ctx.accounts.approver.key(),
        recipient: ctx.accounts.recipient.key(),
        cliff_time,
        amount_per_day,
        escrow_bump,
        update_actor_mode,
        enable_withdrawl_full,
    });
    Ok(())
}
