use anchor_lang::prelude::*;

use crate::constants::MARKET_SEED;
use crate::error::ErrorCode;
use crate::state::Market;

#[derive(Accounts)]
pub struct SetSettlementPrice<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKET_SEED],
        bump = market.bump,
        has_one = authority @ ErrorCode::Unauthorized,
    )]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<SetSettlementPrice>, settlement_price: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;

    require!(market.matched_qty > 0, ErrorCode::InvalidQuantity);
    require!(!market.settled, ErrorCode::AlreadySettled);
    require!(now >= market.expiry_ts, ErrorCode::NotExpired);
    require!(settlement_price > 0, ErrorCode::InvalidSettlementPrice);

    market.settlement_price = settlement_price;
    market.settled = true;

    Ok(())
}
