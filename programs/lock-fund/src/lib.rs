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
declare_id!("5aBQfQ6A6qWVSiQTEweyg9RYLkWgg7BDYh9yScBSP547");

#[program]
pub mod lock_fund {
    use super::*;

    pub fn create_config(
        ctx: Context<CreateConfig>,
        params: CreateConfigParams,
    ) -> Result<()> {
        create_config_handler(ctx, &params)
    }
}
