use anchor_lang::prelude::*;

use crate::constants::{MARKET_SEED, POSITION_SEED};
use crate::error::ErrorCode;
use crate::instructions::matching::sync_matched;
use crate::state::{Market, Position, Side};

#[derive(Accounts)]
pub struct Cancel<'info> {
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

pub fn handler(ctx: Context<Cancel>, side: Side) -> Result<()> {
    require!(!ctx.accounts.market.settled, ErrorCode::AlreadySettled);
    require!(ctx.accounts.position.side == side, ErrorCode::InvalidSide);

    sync_matched(&mut ctx.accounts.position, &ctx.accounts.market)?;

    let position = &ctx.accounts.position;
    let unmatched = position
        .contracts
        .checked_sub(position.matched_contracts)
        .ok_or(ErrorCode::Overflow)?;
    require!(unmatched > 0, ErrorCode::NothingToCancel);

    let refund = ctx
        .accounts
        .market
        .margin_per_contract
        .checked_mul(unmatched)
        .ok_or(ErrorCode::Overflow)?;

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

    let position = &mut ctx.accounts.position;
    position.contracts = position.matched_contracts;
    position.margin = position.matched_margin;

    if position.contracts == 0 {
        ctx.accounts.position.close(ctx.accounts.user.to_account_info())?;
    }

    Ok(())
}
