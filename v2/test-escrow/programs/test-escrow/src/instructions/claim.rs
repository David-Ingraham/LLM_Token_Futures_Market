use anchor_lang::prelude::*;

use crate::constants::{MARKET_SEED, POSITION_SEED};
use crate::error::ErrorCode;
use crate::instructions::matching::{compute_payout, sync_matched};
use crate::state::{Market, Position, Side};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKET_SEED],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [
            POSITION_SEED,
            market.key().as_ref(),
            user.key().as_ref(),
            &[position.side.as_seed_byte()],
        ],
        bump = position.bump,
        constraint = position.owner == user.key() @ ErrorCode::Unauthorized,
    )]
    pub position: Account<'info, Position>,
}

pub fn handler(ctx: Context<Claim>, side: Side) -> Result<()> {
    require!(ctx.accounts.market.settled, ErrorCode::NotSettled);
    require!(ctx.accounts.position.side == side, ErrorCode::InvalidSide);

    if ctx.accounts.position.matched_contracts == 0 {
        sync_matched(&mut ctx.accounts.position, &ctx.accounts.market)?;
    }

    let position = &ctx.accounts.position;
    require!(position.matched_contracts > 0, ErrorCode::NothingToClaim);

    let market = &ctx.accounts.market;
    let payout = compute_payout(
        position.side,
        position.matched_contracts,
        position.matched_margin,
        market.entry_price,
        market.settlement_price,
    )?;

    let market_lamports = ctx.accounts.market.to_account_info().lamports();
    require!(market_lamports >= payout, ErrorCode::InsufficientMarketFunds);

    **ctx
        .accounts
        .market
        .to_account_info()
        .try_borrow_mut_lamports()? -= payout;
    **ctx
        .accounts
        .user
        .to_account_info()
        .try_borrow_mut_lamports()? += payout;

    let unmatched = position
        .contracts
        .checked_sub(position.matched_contracts)
        .ok_or(ErrorCode::Overflow)?;

    if unmatched > 0 {
        let refund = market
            .margin_per_contract
            .checked_mul(unmatched)
            .ok_or(ErrorCode::Overflow)?;

        **ctx
            .accounts
            .market
            .to_account_info()
            .try_borrow_mut_lamports()? -= refund;
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? += refund;

        let market = &mut ctx.accounts.market;
        match side {
            Side::Long => {
                market.long_qty = market.long_qty.checked_sub(unmatched).ok_or(ErrorCode::Overflow)?;
                market.unmatched_long_lamports = market
                    .unmatched_long_lamports
                    .checked_sub(refund)
                    .ok_or(ErrorCode::Overflow)?;
            }
            Side::Short => {
                market.short_qty = market
                    .short_qty
                    .checked_sub(unmatched)
                    .ok_or(ErrorCode::Overflow)?;
                market.unmatched_short_lamports = market
                    .unmatched_short_lamports
                    .checked_sub(refund)
                    .ok_or(ErrorCode::Overflow)?;
            }
        }
    }

    let market = &mut ctx.accounts.market;
    match side {
        Side::Long => {
            market.long_qty = market
                .long_qty
                .checked_sub(position.matched_contracts)
                .ok_or(ErrorCode::Overflow)?;
            market.matched_lamports = market
                .matched_lamports
                .checked_sub(position.matched_margin)
                .ok_or(ErrorCode::Overflow)?;
        }
        Side::Short => {
            market.short_qty = market
                .short_qty
                .checked_sub(position.matched_contracts)
                .ok_or(ErrorCode::Overflow)?;
            market.matched_lamports = market
                .matched_lamports
                .checked_sub(position.matched_margin)
                .ok_or(ErrorCode::Overflow)?;
        }
    }
    ctx.accounts.position.close(ctx.accounts.user.to_account_info())?;

    Ok(())
}
