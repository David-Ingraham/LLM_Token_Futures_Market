use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the position owner may perform this action")]
    Unauthorized,
    #[msg("Quantity must be greater than zero")]
    InvalidQuantity,
    #[msg("Entry price must be greater than zero when matching")]
    InvalidEntryPrice,
    #[msg("Market is already settled")]
    AlreadySettled,
    #[msg("Market is not settled yet")]
    NotSettled,
    #[msg("Market has not expired yet")]
    NotExpired,
    #[msg("Settlement price must be greater than zero")]
    InvalidSettlementPrice,
    #[msg("Position side does not match instruction")]
    InvalidSide,
    #[msg("Insufficient market balance")]
    InsufficientMarketFunds,
    #[msg("Nothing to cancel")]
    NothingToCancel,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Margin per contract must be greater than zero")]
    InvalidMargin,
    #[msg("Arithmetic overflow")]
    Overflow,
}
