use anchor_lang::prelude::*;

use super::Side;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub side: Side,
    pub contracts: u64,
    pub matched_contracts: u64,
    pub margin: u64,
    pub matched_margin: u64,
    pub bump: u8,
}
