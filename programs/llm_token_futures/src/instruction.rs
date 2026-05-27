use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

use crate::error::FuturesError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FuturesInstruction {
    /// accounts: authority, oracle, mint, market, vault, token_program, system, rent
    InitializeMarket {
        market_id: u64,
        entry_price_micro: u64,
        max_settlement_price_micro: u64,
        open_ts: i64,
        trade_cutoff_ts: i64,
        expiry_ts: i64,
        fee_bps: u16,
    },
    /// accounts: owner, market, position, owner_ata, vault, token_program, system
    OpenPosition { side: u8, contracts: u64 },
    /// accounts: crank, market
    HaltTrading,
    /// accounts: oracle, market
    PostSettlementPrice { settlement_price_micro: u64 },
    /// accounts: owner, market, position, owner_ata, vault, token_program, market_authority (same as market PDA)
    SettlePosition,
}

impl FuturesInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Err(FuturesError::InvalidInstruction.into());
        }
        Self::try_from_slice(input).map_err(|_| FuturesError::InvalidInstruction.into())
    }
}

#[cfg(test)]
mod ix_test {
    use super::*;
    use borsh::BorshSerialize;

    #[test]
    fn halt_discriminant() {
        let mut v = Vec::new();
        FuturesInstruction::HaltTrading.serialize(&mut v).unwrap();
        eprintln!("halt ix bytes: {:?}", v);
        assert!(!v.is_empty());
    }
}
