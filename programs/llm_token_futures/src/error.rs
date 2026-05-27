use solana_program::program_error::ProgramError;

#[repr(u32)]
pub enum FuturesError {
    InvalidInstruction = 6000,
    InvalidPrice,
    InvalidSchedule,
    InvalidFee,
    InvalidContracts,
    InvalidSide,
    MarketNotOpen,
    TradingHalted,
    CutoffNotReached,
    InvalidMarketStatus,
    UnauthorizedOracle,
    ExpiryNotReached,
    PriceAboveMax,
    PriceAlreadyPosted,
    PriceNotPosted,
    PositionAlreadySettled,
    UnauthorizedOwner,
    SideMismatch,
    MathOverflow,
    PayoutNegative,
    MintMismatch,
    VaultMismatch,
    MarketMismatch,
    UninitializedAccount,
}

impl From<FuturesError> for ProgramError {
    fn from(e: FuturesError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
