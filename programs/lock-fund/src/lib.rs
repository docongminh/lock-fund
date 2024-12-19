use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod event;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use event::*;
pub use instructions::*;
pub use state::*;
declare_id!("9brXu1LNCURaR8bde1ekTueEnafQ1nHDn1Y2BTfzCKyK");

#[program]
pub mod lock_fund {
    use super::*;

    pub fn create_lock_fund_escrow(
        ctx: Context<CreateLockFundEscrow>,
        params: CreateEscrowFundParams,
    ) -> Result<()> {
        create_lock_fund_escrow_handler(ctx, &params)
    }
}
