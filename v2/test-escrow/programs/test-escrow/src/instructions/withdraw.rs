use anchor_lang::prelude::*;

use crate::constants::ESCROW_SEED;
use crate::error::ErrorCode;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        close = depositor,
        seeds = [ESCROW_SEED, depositor.key().as_ref()],
        bump = escrow.bump,
        has_one = depositor @ ErrorCode::Unauthorized,
    )]
    pub escrow: Account<'info, Escrow>,
}

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    require!(
        Clock::get()?.unix_timestamp >= ctx.accounts.escrow.unlock_time,
        ErrorCode::TooEarly
    );

    Ok(())
}
