use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::constants::{MARKET_SEED, POSITION_SEED};
use crate::error::ErrorCode;
use crate::instructions::matching::try_match;
use crate::state::{Market, Position, Side};

#[derive(Accounts)]
#[instruction(side: Side)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKET_SEED],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Position::INIT_SPACE,
        seeds = [
            POSITION_SEED,
            market.key().as_ref(),
            user.key().as_ref(),
            &[side.as_seed_byte()],
        ],
        bump,
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<OpenPosition>, side: Side, qty: u64, entry_price: u64) -> Result<()> {
    require!(qty > 0, ErrorCode::InvalidQuantity);
    require!(!ctx.accounts.market.settled, ErrorCode::AlreadySettled);

    let margin = ctx
        .accounts
        .market
        .margin_per_contract
        .checked_mul(qty)
        .ok_or(ErrorCode::Overflow)?;

    let position = &mut ctx.accounts.position;
    if position.owner == Pubkey::default() {
        position.owner = ctx.accounts.user.key();
        position.side = side;
        position.bump = ctx.bumps.position;
    } else {
        require!(position.side == side, ErrorCode::InvalidSide);
        require!(position.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
    }

    position.contracts = position
        .contracts
        .checked_add(qty)
        .ok_or(ErrorCode::Overflow)?;
    position.margin = position.margin.checked_add(margin).ok_or(ErrorCode::Overflow)?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.market.to_account_info(),
            },
        ),
        margin,
    )?;

    let market = &mut ctx.accounts.market;
    match side {
        Side::Long => {
            market.long_qty = market.long_qty.checked_add(qty).ok_or(ErrorCode::Overflow)?;
            market.unmatched_long_lamports = market
                .unmatched_long_lamports
                .checked_add(margin)
                .ok_or(ErrorCode::Overflow)?;
        }
        Side::Short => {
            market.short_qty = market.short_qty.checked_add(qty).ok_or(ErrorCode::Overflow)?;
            market.unmatched_short_lamports = market
                .unmatched_short_lamports
                .checked_add(margin)
                .ok_or(ErrorCode::Overflow)?;
        }
    }

    try_match(market, position, qty, entry_price)?;

    Ok(())
}
