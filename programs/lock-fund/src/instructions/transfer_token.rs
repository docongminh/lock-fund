use anchor_spl::{
    token::{TransferChecked, ID},
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked as TransferChecked2022},
};

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
    pub escrow_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Recipient Token Account.
    #[account(
        mut,
        associated_token::mint = mint_token,
        associated_token::authority = recipient
    )]
    pub recipient_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This account use to validate transfer recipient
    pub recipient: AccountInfo<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_token: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub approver: Signer<'info>,
    /// Token program.
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn transfer_token_handler(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
    let config_account = ctx.accounts.config_account.load()?;
    let escrow_seeds = escrow_seeds!(config_account);
    let token_program = &ctx.accounts.token_program;

    let decimals = ctx.accounts.mint_token.decimals;
    if token_program.key().eq(&ID.key()) {
        anchor_spl::token::transfer_checked(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.escrow_token.to_account_info(),
                    mint: ctx.accounts.mint_token.to_account_info(),
                    to: ctx.accounts.recipient_token.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                &[&escrow_seeds[..]],
            ),
            amount,
            decimals,
        )?;
    } else {
        anchor_spl::token_2022::transfer_checked(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                TransferChecked2022 {
                    from: ctx.accounts.escrow_token.to_account_info(),
                    mint: ctx.accounts.mint_token.to_account_info(),
                    to: ctx.accounts.recipient_token.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                &[&escrow_seeds[..]],
            ),
            amount,
            decimals,
        )?;
    }

    emit_cpi!(TransferEvent {
        from: ctx.accounts.escrow_token.key(),
        to: ctx.accounts.recipient_token.key(),
        config_account: ctx.accounts.config_account.key(),
        amount: amount
    });

    Ok(())
}
