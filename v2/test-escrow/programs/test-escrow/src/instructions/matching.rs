use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::{Market, Position, Side};

pub fn sync_matched(position: &mut Position, market: &Market) -> Result<()> {
    let side_qty = match position.side {
        Side::Long => market.long_qty,
        Side::Short => market.short_qty,
    };

    if side_qty == 0 {
        position.matched_contracts = 0;
        position.matched_margin = 0;
        return Ok(());
    }

    position.matched_contracts = (position.contracts as u128)
        .checked_mul(market.matched_qty as u128)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(side_qty as u128)
        .ok_or(ErrorCode::Overflow)? as u64;

    position.matched_margin = position
        .matched_contracts
        .checked_mul(market.margin_per_contract)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}

pub fn try_match(
    market: &mut Market,
    position: &mut Position,
    qty: u64,
    entry_price: u64,
) -> Result<()> {
    let new_matched = market
        .long_qty
        .min(market.short_qty)
        .saturating_sub(market.matched_qty);

    if new_matched == 0 {
        return Ok(());
    }

    require!(entry_price > 0, ErrorCode::InvalidEntryPrice);

    if market.entry_price == 0 {
        market.entry_price = entry_price;
    }

    let margin_per = market.margin_per_contract;
    let long_move = margin_per
        .checked_mul(new_matched)
        .ok_or(ErrorCode::Overflow)?;
    let short_move = long_move;

    market.unmatched_long_lamports = market
        .unmatched_long_lamports
        .checked_sub(long_move)
        .ok_or(ErrorCode::Overflow)?;
    market.unmatched_short_lamports = market
        .unmatched_short_lamports
        .checked_sub(short_move)
        .ok_or(ErrorCode::Overflow)?;
    market.matched_lamports = market
        .matched_lamports
        .checked_add(
            long_move
                .checked_add(short_move)
                .ok_or(ErrorCode::Overflow)?,
        )
        .ok_or(ErrorCode::Overflow)?;
    market.matched_qty = market
        .matched_qty
        .checked_add(new_matched)
        .ok_or(ErrorCode::Overflow)?;

    // Credit the opener; passive-side positions sync at claim if still zero.
    let assign = new_matched.min(qty);
    if assign > 0 {
        let margin = margin_per.checked_mul(assign).ok_or(ErrorCode::Overflow)?;
        position.matched_contracts = position
            .matched_contracts
            .checked_add(assign)
            .ok_or(ErrorCode::Overflow)?;
        position.matched_margin = position
            .matched_margin
            .checked_add(margin)
            .ok_or(ErrorCode::Overflow)?;
    }

    Ok(())
}

pub fn compute_payout(
    side: Side,
    matched_contracts: u64,
    matched_margin: u64,
    entry_price: u64,
    settlement_price: u64,
) -> Result<u64> {
    let entry = entry_price as i64;
    let settlement = settlement_price as i64;
    let qty = matched_contracts as i64;

    let pnl_per_contract = match side {
        Side::Long => settlement.checked_sub(entry).ok_or(ErrorCode::Overflow)?,
        Side::Short => entry.checked_sub(settlement).ok_or(ErrorCode::Overflow)?,
    };

    let pnl = pnl_per_contract
        .checked_mul(qty)
        .ok_or(ErrorCode::Overflow)?;

    let payout = (matched_margin as i64)
        .checked_add(pnl)
        .ok_or(ErrorCode::Overflow)?;

    Ok(payout.max(0) as u64)
}
