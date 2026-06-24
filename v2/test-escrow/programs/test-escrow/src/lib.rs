pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("83hEVUBiTxp2H2cHHVLCXLT7jobKzcmgBcEjiN1tnNHD");

#[program]
pub mod test_escrow {
    use super::*;

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        expiry_ts: i64,
        margin_per_contract: u64,
    ) -> Result<()> {
        initialize_market::handler(ctx, expiry_ts, margin_per_contract)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        side: Side,
        qty: u64,
        entry_price: u64,
    ) -> Result<()> {
        open_position::handler(ctx, side, qty, entry_price)
    }

    pub fn cancel(ctx: Context<Cancel>, side: Side) -> Result<()> {
        cancel::handler(ctx, side)
    }

    pub fn set_settlement_price(
        ctx: Context<SetSettlementPrice>,
        settlement_price: u64,
    ) -> Result<()> {
        set_settlement_price::handler(ctx, settlement_price)
    }

    pub fn claim(ctx: Context<Claim>, side: Side) -> Result<()> {
        claim::handler(ctx, side)
    }
}
