use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::*;

#[event_cpi]
#[derive(Accounts)]
pub struct TransferToken<'info> {
    /// Escrow.
    #[account(mut)]
    pub config_account: AccountLoader<'info, ConfigAccount>,

    /// CHECK: This account use to validate escrow_token
    pub escrow: AccountInfo<'info>,

    /// CHECK: Escrow Token Account.
    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = escrow
    )]
    pub escrow_token: Account<'info, TokenAccount>,

    /// CHECK: Recipient Token Account.
    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = recipient
    )]
    pub recipient_token: Account<'info, TokenAccount>,

    /// CHECK: This account use to validate transfer recipient
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub approver: Signer<'info>,
    /// Token program.
    pub token_program: Program<'info, Token>,
}

pub fn transfer_token_handler(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
    let config_account = ctx.accounts.config_account.load()?;
    let escrow_seeds = escrow_seeds!(config_account);

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.escrow_token.to_account_info(),
                to: ctx.accounts.recipient_token.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            &[&escrow_seeds[..]],
        ),
        amount,
    )?;

    emit_cpi!(TransferEvent {
        from: ctx.accounts.escrow_token.key(),
        to: ctx.accounts.recipient_token.key(),
        config_account: ctx.accounts.config_account.key(),
        amount: amount
    });

    Ok(())
}
