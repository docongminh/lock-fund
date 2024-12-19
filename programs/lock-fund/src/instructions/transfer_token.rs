use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::*;

#[event_cpi]
#[derive(Accounts)]
pub struct TransferToken<'info> {
    /// Escrow.
    #[account(mut)]
    pub config_account: AccountLoader<'info, ConfigAccount>,

    // pub escrow_vault: AccountInfo<'info>,

    // pub recipient_token_account: AccountInfo<'info>,

    // pub authority: Signer<'info>,
    // pub approver: Signer<'info>,

    /// Token program.
    pub token_program: Program<'info, Token>,
}

pub fn transfer_token_handler(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
    // let escrow_state = escrow.load()?;
    // let escrow_seeds = escrow_seeds!(escrow_state);

    // anchor_spl::token::transfer(
    //     CpiContext::new_with_signer(
    //         token_program.to_account_info(),
    //         Transfer {
    //             from: escrow_token.to_account_info(),
    //             to: recipient_token.to_account_info(),
    //             authority: escrow.to_account_info(),
    //         },
    //         &[&escrow_seeds[..]],
    //     ),
    //     amount,
    // )?;

    Ok(())
}
