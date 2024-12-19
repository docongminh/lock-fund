use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum LockFundEscrowError {
    #[msg("unauthorize for this action")]
    Unauthorize,
    #[msg("Two pubkey can not duplicate")]
    DuplicatePubkey,
    #[msg("nvalid Recipient")]
    InvalidRecipient
}