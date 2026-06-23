use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Withdrawal is not yet allowed")]
    TooEarly,
    #[msg("Only the depositor may withdraw")]
    Unauthorized,
    #[msg("Deposit amount must be greater than zero")]
    InvalidAmount,
    #[msg("Pool balance is insufficient")]
    InsufficientPoolFunds,
}
