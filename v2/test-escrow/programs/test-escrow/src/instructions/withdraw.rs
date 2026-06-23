use anchor_lang::prelude::*;

use crate::constants::{POOL_SEED, USER_DEPOSIT_SEED};
use crate::error::ErrorCode;
use crate::state::{Pool, UserDeposit};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [POOL_SEED],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        close = depositor,
        seeds = [USER_DEPOSIT_SEED, depositor.key().as_ref()],
        bump = user_deposit.bump,
        has_one = depositor @ ErrorCode::Unauthorized,
    )]
    pub user_deposit: Account<'info, UserDeposit>,
}

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    let user_deposit = &ctx.accounts.user_deposit;
    require!(
        Clock::get()?.unix_timestamp >= user_deposit.unlock_time,
        ErrorCode::TooEarly
    );

    let amount = user_deposit.amount;
    let pool = &mut ctx.accounts.pool;
    pool.total_deposited = pool
        .total_deposited
        .checked_sub(amount)
        .ok_or(ErrorCode::InsufficientPoolFunds)?;

    **ctx
        .accounts
        .pool
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .depositor
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    Ok(())
}
