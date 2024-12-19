use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum UpdateActorMode {
    None = 0,
    Authority = 1_u8 << 0, // 00000001
    Approver = 1_u8 << 1,  //  00000010
    Recipient = 1_u8 << 2, //  00000100
}

#[account(zero_copy)]
#[derive(InitSpace)]
pub struct ConfigAccount {
    // Authority of this lock fund escrow aka creator
    pub authority: Pubkey,
    // Multi-sign to increase secure for withdraw fund action
    pub approver: Pubkey,
    // Recipient fund address
    pub recipient: Pubkey,
    // escrow vault
    pub escrow: Pubkey,
    /// Cliff time: After the cliff time, the actor can withdraw funds
    pub cliff_time: u64,
    // Max amount that can be withdrawn per day to prevent draining all funds in case of a vulnerability
    pub amount_per_day: u64,
    // Mode allows fields to be updatable
    pub update_actor_mode: u8,
    // 1: Allow, 0: Deny
    pub enable_transfer_full: u8,
    // Escrow bump
    pub config_bump: u8,
    // Escrow vault bump
    pub escrow_bump: u8,
    // padding for alignment
    pub padding_0: [u8; 12],
}

const_assert_eq!(ConfigAccount::INIT_SPACE, 160);

impl ConfigAccount {
    pub fn init(
        &mut self,
        authority: Pubkey,
        approver: Pubkey,
        recipient: Pubkey,
        escrow: Pubkey,
        cliff_time: u64,
        amount_per_day: u64,
        update_actor_mode: u8,
        enable_transfer_full: u8,
        config_bump: u8,
        escrow_bump: u8,
    ) {
        self.authority = authority;
        self.approver = approver;
        self.recipient = recipient;
        self.cliff_time = cliff_time;
        self.amount_per_day = amount_per_day;
        self.update_actor_mode = update_actor_mode;
        self.enable_transfer_full = enable_transfer_full;
        self.escrow = escrow;
        self.config_bump = config_bump;
        self.escrow_bump = escrow_bump;
    }
}
