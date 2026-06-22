use crate::error::FuturesError;
use crate::state::Side;
use solana_program::program_error::ProgramError;

pub fn required_margin_micro(
    side: u8,
    contracts: u64,
    entry_price_micro: u64,
    max_settlement_price_micro: u64,
    contract_mtok: u64,
) -> Result<u64, ProgramError> {
    let product = contracts
        .checked_mul(contract_mtok)
        .ok_or(FuturesError::MathOverflow)?;
    if side == Side::Long as u8 {
        return entry_price_micro
            .checked_mul(product)
            .ok_or(FuturesError::MathOverflow.into());
    }
    let upside = max_settlement_price_micro
        .checked_sub(entry_price_micro)
        .ok_or(FuturesError::InvalidPrice)?;
    upside
        .checked_mul(product)
        .ok_or(FuturesError::MathOverflow.into())
}

pub fn settlement_pnl_micro(
    side: u8,
    contracts: u64,
    entry_price_micro: u64,
    settlement_price_micro: u64,
    contract_mtok: u64,
) -> Result<i64, ProgramError> {
    let delta = settlement_price_micro as i64 - entry_price_micro as i64;
    let raw = (contracts as i64)
        .checked_mul(delta)
        .and_then(|v| v.checked_mul(contract_mtok as i64))
        .ok_or(FuturesError::MathOverflow)?;
    if side == Side::Long as u8 {
        Ok(raw)
    } else {
        Ok(-raw)
    }
}

pub fn settlement_fee_micro(pnl: i64, fee_bps: u16) -> Result<u64, ProgramError> {
    if fee_bps == 0 || pnl == 0 {
        return Ok(0);
    }
    let abs_pnl = pnl.unsigned_abs();
    Ok(abs_pnl
        .checked_mul(fee_bps as u64)
        .and_then(|v| v.checked_div(10_000))
        .ok_or(FuturesError::MathOverflow)?)
}

pub fn settlement_payout_micro(
    locked_margin: u64,
    pnl: i64,
    fee: u64,
) -> Result<u64, ProgramError> {
    let margin_i = locked_margin as i64;
    let payout_i = margin_i
        .checked_add(pnl)
        .and_then(|v| v.checked_sub(fee as i64))
        .ok_or(FuturesError::MathOverflow)?;
    if payout_i < 0 {
        return Err(FuturesError::PayoutNegative.into());
    }
    Ok(payout_i as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_pnl() {
        assert_eq!(
            settlement_pnl_micro(0, 10, 25_000_000, 27_000_000, 1).unwrap(),
            20_000_000
        );
    }
}
