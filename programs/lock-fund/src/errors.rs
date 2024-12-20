use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum LockFundEscrowError {
    #[msg("unauthorize for this action")]
    Unauthorize,
    #[msg("Two pubkey can not duplicate")]
    DuplicatePubkey,
    #[msg("invalid escrow")]
    InvalidEscrow,
    #[msg("invalid Recipient")]
    InvalidRecipient
}