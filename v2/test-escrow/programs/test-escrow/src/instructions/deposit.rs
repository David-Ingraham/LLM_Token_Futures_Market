use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::constants::{LOCK_DURATION, POOL_SEED, USER_DEPOSIT_SEED};
use crate::error::ErrorCode;
use crate::state::{Pool, UserDeposit};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Pool::INIT_SPACE,
        seeds = [POOL_SEED],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        space = 8 + UserDeposit::INIT_SPACE,
        seeds = [USER_DEPOSIT_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_deposit: Account<'info, UserDeposit>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    let pool = &mut ctx.accounts.pool;
    if pool.bump == 0 {
        pool.bump = ctx.bumps.pool;
    }
    pool.total_deposited = pool
        .total_deposited
        .checked_add(amount)
        .ok_or(ErrorCode::InsufficientPoolFunds)?;

    let user_deposit = &mut ctx.accounts.user_deposit;
    user_deposit.depositor = ctx.accounts.user.key();
    user_deposit.amount = amount;
    user_deposit.bump = ctx.bumps.user_deposit;
    user_deposit.unlock_time = Clock::get()?.unix_timestamp + LOCK_DURATION;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.pool.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
