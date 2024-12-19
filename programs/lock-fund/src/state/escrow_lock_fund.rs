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
pub struct LockFundEscrow {
    // Authority of this lock fund escrow aka creator
    pub authority: Pubkey,
    // Multi-sign to increase secure for withdraw fund action
    pub approver: Pubkey,
    // Recipient fund address
    pub recipient: Pubkey,
    /// Cliff time: After the cliff time, the actor can withdraw funds
    pub cliff_time: u64,
    // Max amount that can be withdrawn per day to prevent draining all funds in case of a vulnerability
    pub amount_per_day: u64,
    // Escrow bump
    pub escrow_bump: u8,
    // Mode allows fields to be updatable
    pub update_actor_mode: u8,
    // 1: Allow, 0: Deny
    pub enable_withdrawl_full: u8,
    // padding for alignment
    pub padding_0: [u8; 5],
}

const_assert_eq!(LockFundEscrow::INIT_SPACE, 120); // 32 * 3 + 64 * 2 + 8

impl LockFundEscrow {
    pub fn init(
        &mut self,
        authority: Pubkey,
        approver: Pubkey,
        recipient: Pubkey,
        cliff_time: u64,
        amount_per_day: u64,
        escrow_bump: u8,
        update_actor_mode: u8,
        enable_withdrawl_full: u8,
    ) {
        self.authority = authority;
        self.approver = approver;
        self.recipient = recipient;
        self.cliff_time = cliff_time;
        self.amount_per_day = amount_per_day;
        self.escrow_bump = escrow_bump;
        self.update_actor_mode = update_actor_mode;
        self.enable_withdrawl_full = enable_withdrawl_full;
    }
}
