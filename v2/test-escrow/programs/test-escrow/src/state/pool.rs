use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub total_deposited: u64,
    pub bump: u8,
}
