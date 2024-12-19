use anchor_lang::prelude::*;

#[event]
pub struct CreateLockFundEscrowEvent {
    pub authority: Pubkey,
    pub approver: Pubkey,
    pub recipient: Pubkey,
    pub cliff_time: u64,
    pub amount_per_day: u64,
    pub escrow_bump: u8,
    pub update_actor_mode: u8,
    pub enable_withdrawl_full: u8,
}