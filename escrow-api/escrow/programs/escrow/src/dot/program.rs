#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

seahorse_const! { LOCK_SECONDS , 300 }

#[account]
#[derive(Debug)]
pub struct Escrow {
    pub owner: Pubkey,
    pub amount: u64,
    pub unlock_at: i64,
    pub bump: u8,
}

impl<'info, 'entrypoint> Escrow {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedEscrow<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let amount = account.amount;
        let unlock_at = account.unlock_at;
        let bump = account.bump;

        Mutable::new(LoadedEscrow {
            __account__: account,
            __programs__: programs_map,
            owner,
            amount,
            unlock_at,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedEscrow>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let amount = loaded.amount;

        loaded.__account__.amount = amount;

        let unlock_at = loaded.unlock_at;

        loaded.__account__.unlock_at = unlock_at;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedEscrow<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Escrow>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub amount: u64,
    pub unlock_at: i64,
    pub bump: u8,
}

pub fn deposit_handler<'info>(
    mut depositor: SeahorseSigner<'info, '_>,
    mut escrow: Empty<Mutable<LoadedEscrow<'info, '_>>>,
    mut amount: u64,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    if !(amount > 0) {
        panic!("amount must be positive");
    }

    let mut bump = escrow.bump.unwrap();
    let mut vault = escrow.account.clone();

    assign!(vault.borrow_mut().owner, depositor.key());

    assign!(vault.borrow_mut().amount, amount);

    assign!(
        vault.borrow_mut().unlock_at,
        clock.unix_timestamp + LOCK_SECONDS!()
    );

    assign!(vault.borrow_mut().bump, bump);

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &depositor.key(),
            &vault.borrow().__account__.key(),
            amount.clone(),
        ),
        &[
            depositor.to_account_info(),
            vault.borrow().__account__.to_account_info(),
            depositor.programs.get("system_program").clone(),
        ],
    )
    .unwrap();
}

pub fn release_handler<'info>(
    mut depositor: SeahorseSigner<'info, '_>,
    mut escrow: Mutable<LoadedEscrow<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    if !(depositor.key() == escrow.borrow().owner) {
        panic!("not the depositor");
    }

    if !(clock.unix_timestamp >= escrow.borrow().unlock_at) {
        panic!("escrow still locked");
    }

    if !(escrow.borrow().amount > 0) {
        panic!("nothing to release");
    }

    let mut amount = escrow.borrow().amount;

    assign!(escrow.borrow_mut().amount, 0);

    {
        let amount = amount.clone();

        **escrow
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **depositor
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };
}
