use anchor_lang::prelude::*;

use crate::constants::MARKET_SEED;
use crate::error::ErrorCode;
use crate::state::Market;

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Market::INIT_SPACE,
        seeds = [MARKET_SEED],
        bump,
    )]
    pub market: Account<'info, Market>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMarket>,
    expiry_ts: i64,
    margin_per_contract: u64,
) -> Result<()> {
    require!(margin_per_contract > 0, ErrorCode::InvalidMargin);

    let market = &mut ctx.accounts.market;
    market.authority = ctx.accounts.authority.key();
    market.expiry_ts = expiry_ts;
    market.entry_price = 0;
    market.settlement_price = 0;
    market.long_qty = 0;
    market.short_qty = 0;
    market.matched_qty = 0;
    market.unmatched_long_lamports = 0;
    market.unmatched_short_lamports = 0;
    market.matched_lamports = 0;
    market.margin_per_contract = margin_per_contract;
    market.settled = false;
    market.bump = ctx.bumps.market;

    Ok(())
}
