use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub expiry_ts: i64,
    pub entry_price: u64,
    pub settlement_price: u64,
    pub long_qty: u64,
    pub short_qty: u64,
    pub matched_qty: u64,
    pub unmatched_long_lamports: u64,
    pub unmatched_short_lamports: u64,
    pub matched_lamports: u64,
    pub margin_per_contract: u64,
    pub settled: bool,
    pub bump: u8,
}
