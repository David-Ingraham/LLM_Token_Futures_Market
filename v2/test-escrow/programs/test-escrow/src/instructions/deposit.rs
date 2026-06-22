use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::constants::{ESCROW_SEED, LOCK_DURATION};
use crate::error::ErrorCode;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, user.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    let escrow = &mut ctx.accounts.escrow;
    escrow.depositor = ctx.accounts.user.key();
    escrow.bump = ctx.bumps.escrow;
    escrow.unlock_time = Clock::get()?.unix_timestamp + LOCK_DURATION;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
