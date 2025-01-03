use crate::*;

#[event_cpi]
#[derive(Accounts)]
pub struct TransferSol<'info> {

    #[account(mut, has_one = authority, has_one = approver)]
    pub config_account: AccountLoader<'info, ConfigAccount>,

    /// CHECK: account will transfer fund
    #[account(mut, constraint = escrow.key() == config_account.load()?.escrow @ LockFundEscrowError::InvalidEscrow)]
    pub escrow: AccountInfo<'info>,

    /// CHECK: account will receive fund
    #[account(mut, constraint = recipient.key() == config_account.load()?.recipient @ LockFundEscrowError::InvalidRecipient)]
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub approver: Signer<'info>,
    /// system program.
    pub system_program: Program<'info, System>,
}

pub fn transfer_sol_handler(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
    let config_account = ctx.accounts.config_account.load()?;
    let escrow_seeds = escrow_seeds!(config_account);

    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.escrow.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
            &[&escrow_seeds[..]],
        ),
        amount,
    )?;

    emit_cpi!(TransferEvent {
        from: ctx.accounts.escrow.key(),
        to: ctx.accounts.recipient.key(),
        config_account: ctx.accounts.config_account.key(),
        amount: amount
    });

    Ok(())
}
