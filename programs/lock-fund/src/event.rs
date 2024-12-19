use anchor_lang::prelude::*;

#[event]
pub struct CreateConfigEvent {
    pub authority: Pubkey,
    pub approver: Pubkey,
    pub recipient: Pubkey,
    pub cliff_time_duration: u64,
    pub amount_per_day: u64,
    pub update_actor_mode: u8,
    pub enable_transfer_full: u8,
}

#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub config_account: Pubkey,
    pub amount: u64
}
