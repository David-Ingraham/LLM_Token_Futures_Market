use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use crate::error::FuturesError;
use crate::instruction::FuturesInstruction;
use crate::math::{
    required_margin_micro, settlement_fee_micro, settlement_payout_micro, settlement_pnl_micro,
};
use crate::state::{Market, MarketStatus, Position, Side, MARKET_LEN, MARKET_MAGIC, POSITION_LEN, POSITION_MAGIC};

pub const DEFAULT_CONTRACT_MTOK: u64 = 1;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = FuturesInstruction::unpack(instruction_data)?;
    match ix {
        FuturesInstruction::InitializeMarket {
            market_id,
            entry_price_micro,
            max_settlement_price_micro,
            open_ts,
            trade_cutoff_ts,
            expiry_ts,
            fee_bps,
        } => initialize_market(
            program_id,
            accounts,
            market_id,
            entry_price_micro,
            max_settlement_price_micro,
            open_ts,
            trade_cutoff_ts,
            expiry_ts,
            fee_bps,
        ),
        FuturesInstruction::OpenPosition { side, contracts } => {
            open_position(program_id, accounts, side, contracts)
        }
        FuturesInstruction::HaltTrading => halt_trading(accounts),
        FuturesInstruction::PostSettlementPrice {
            settlement_price_micro,
        } => post_settlement_price(accounts, settlement_price_micro),
        FuturesInstruction::SettlePosition => settle_position(program_id, accounts),
    }
}

fn initialize_market(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    market_id: u64,
    entry_price_micro: u64,
    max_settlement_price_micro: u64,
    open_ts: i64,
    trade_cutoff_ts: i64,
    expiry_ts: i64,
    fee_bps: u16,
) -> ProgramResult {
    if entry_price_micro == 0 {
        return Err(FuturesError::InvalidPrice.into());
    }
    if entry_price_micro > max_settlement_price_micro {
        return Err(FuturesError::InvalidPrice.into());
    }
    if open_ts >= trade_cutoff_ts || trade_cutoff_ts > expiry_ts {
        return Err(FuturesError::InvalidSchedule.into());
    }
    if fee_bps > 10_000 {
        return Err(FuturesError::InvalidFee.into());
    }

    let account_info_iter = &mut accounts.iter();
    let authority = next_account_info(account_info_iter)?;
    let oracle = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let market = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let market_id_bytes = market_id.to_le_bytes();
    let (market_pda, market_bump) = Pubkey::find_program_address(
        &[
            b"market",
            authority.key.as_ref(),
            market_id_bytes.as_ref(),
        ],
        program_id,
    );
    if market_pda != *market.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let (vault_pda, vault_bump) =
        Pubkey::find_program_address(&[b"vault", market.key.as_ref()], program_id);
    if vault_pda != *vault.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let rent = Rent::from_account_info(rent_sysvar)?;

    let market_lamports = rent.minimum_balance(MARKET_LEN);
    invoke_signed(
        &system_instruction::create_account(
            authority.key,
            market.key,
            market_lamports,
            MARKET_LEN as u64,
            program_id,
        ),
        &[authority.clone(), market.clone(), system_program.clone()],
        &[&[
            b"market",
            authority.key.as_ref(),
            &market_id.to_le_bytes(),
            &[market_bump],
        ]],
    )?;

    let vault_lamports = rent.minimum_balance(crate::spl_token::ACCOUNT_LEN);
    invoke_signed(
        &system_instruction::create_account(
            authority.key,
            vault.key,
            vault_lamports,
            crate::spl_token::ACCOUNT_LEN as u64,
            &crate::spl_token::ID,
        ),
        &[authority.clone(), vault.clone(), system_program.clone()],
        &[&[b"vault", market.key.as_ref(), &[vault_bump]]],
    )?;

    crate::spl_token::initialize_account(
        token_program,
        vault,
        mint,
        market,
        rent_sysvar,
        &[&[
            b"market",
            authority.key.as_ref(),
            &market_id.to_le_bytes(),
            &[market_bump],
        ]],
    )?;

    let data = Market {
        magic: MARKET_MAGIC,
        authority: *authority.key,
        oracle: *oracle.key,
        mint: *mint.key,
        vault: *vault.key,
        market_id,
        entry_price_micro,
        max_settlement_price_micro,
        settlement_price_micro: 0,
        open_ts,
        trade_cutoff_ts,
        expiry_ts,
        fee_bps,
        contract_mtok: DEFAULT_CONTRACT_MTOK,
        status: MarketStatus::Open as u8,
        bump: market_bump,
        vault_bump,
    };
    data.pack_into_slice(&mut market.data.borrow_mut());

    Ok(())
}

fn open_position(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    side: u8,
    contracts: u64,
) -> ProgramResult {
    if contracts == 0 {
        return Err(FuturesError::InvalidContracts.into());
    }
    if side != Side::Long as u8 && side != Side::Short as u8 {
        return Err(FuturesError::InvalidSide.into());
    }

    let account_info_iter = &mut accounts.iter();
    let owner = next_account_info(account_info_iter)?;
    let market = next_account_info(account_info_iter)?;
    let position = next_account_info(account_info_iter)?;
    let owner_ata = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let market_state = load_market(&market.data.borrow())?;
    if market_state.magic != MARKET_MAGIC {
        return Err(FuturesError::UninitializedAccount.into());
    }
    if *vault.key != market_state.vault {
        return Err(FuturesError::VaultMismatch.into());
    }

    let clock = Clock::get()?;
    if market_state.status != MarketStatus::Open as u8 {
        return Err(FuturesError::MarketNotOpen.into());
    }
    if clock.unix_timestamp < market_state.open_ts {
        return Err(FuturesError::MarketNotOpen.into());
    }
    if clock.unix_timestamp >= market_state.trade_cutoff_ts {
        return Err(FuturesError::TradingHalted.into());
    }

    let margin = required_margin_micro(
        side,
        contracts,
        market_state.entry_price_micro,
        market_state.max_settlement_price_micro,
        market_state.contract_mtok,
    )?;

    let (position_pda, position_bump) = Pubkey::find_program_address(
        &[
            b"position",
            market.key.as_ref(),
            owner.key.as_ref(),
        ],
        program_id,
    );
    if position_pda != *position.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let mut position_data = position.data.borrow_mut();
    let mut position_state = if position_data.len() >= POSITION_LEN {
        Position::unpack(&position_data)?
    } else {
        Position {
            magic: 0,
            owner: Pubkey::default(),
            market: Pubkey::default(),
            side: 0,
            contracts: 0,
            locked_margin: 0,
            settled: 1,
            bump: 0,
        }
    };

    if !position_state.is_initialized() {
        let rent = Rent::from_account_info(rent_sysvar)?;
        invoke_signed(
            &system_instruction::create_account(
                owner.key,
                position.key,
                rent.minimum_balance(POSITION_LEN),
                POSITION_LEN as u64,
                program_id,
            ),
            &[owner.clone(), position.clone(), system_program.clone()],
            &[&[
                b"position",
                market.key.as_ref(),
                owner.key.as_ref(),
                &[position_bump],
            ]],
        )?;
        position_state = Position {
            magic: POSITION_MAGIC,
            owner: *owner.key,
            market: *market.key,
            side,
            contracts,
            locked_margin: margin,
            settled: 0,
            bump: position_bump,
        };
    } else {
        if position_state.settled == 0 && position_state.side != side {
            return Err(FuturesError::SideMismatch.into());
        }
        if position_state.settled != 0 {
            return Err(FuturesError::PositionAlreadySettled.into());
        }
        position_state.contracts = position_state
            .contracts
            .checked_add(contracts)
            .ok_or(FuturesError::MathOverflow)?;
        position_state.locked_margin = position_state
            .locked_margin
            .checked_add(margin)
            .ok_or(FuturesError::MathOverflow)?;
    }

    crate::spl_token::transfer(
        token_program,
        owner_ata,
        vault,
        owner,
        &[],
        margin,
    )?;

    position_state.pack_into_slice(&mut position_data);
    Ok(())
}

fn halt_trading(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let _crank = next_account_info(account_info_iter)?;
    let market = next_account_info(account_info_iter)?;

    let mut market_state = load_market_mut(market)?;
    let clock = Clock::get()?;
    if clock.unix_timestamp < market_state.trade_cutoff_ts {
        return Err(FuturesError::CutoffNotReached.into());
    }
    if market_state.status != MarketStatus::Open as u8 {
        return Err(FuturesError::InvalidMarketStatus.into());
    }
    market_state.status = MarketStatus::Halted as u8;
    market_state.pack_into_slice(&mut market.data.borrow_mut());
    Ok(())
}

fn post_settlement_price(
    accounts: &[AccountInfo],
    settlement_price_micro: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let oracle = next_account_info(account_info_iter)?;
    let market = next_account_info(account_info_iter)?;

    if !oracle.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut market_state = load_market_mut(market)?;
    if oracle.key != &market_state.oracle {
        return Err(FuturesError::UnauthorizedOracle.into());
    }
    let clock = Clock::get()?;
    if clock.unix_timestamp < market_state.expiry_ts {
        return Err(FuturesError::ExpiryNotReached.into());
    }
    if market_state.status != MarketStatus::Halted as u8 {
        return Err(FuturesError::InvalidMarketStatus.into());
    }
    if settlement_price_micro > market_state.max_settlement_price_micro {
        return Err(FuturesError::PriceAboveMax.into());
    }
    if market_state.settlement_price_micro != 0 {
        return Err(FuturesError::PriceAlreadyPosted.into());
    }

    market_state.settlement_price_micro = settlement_price_micro;
    market_state.status = MarketStatus::PricePosted as u8;
    market_state.pack_into_slice(&mut market.data.borrow_mut());
    Ok(())
}

fn settle_position(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner = next_account_info(account_info_iter)?;
    let market = next_account_info(account_info_iter)?;
    let position = next_account_info(account_info_iter)?;
    let owner_ata = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let market_state = load_market(&market.data.borrow())?;
    if market_state.status != MarketStatus::PricePosted as u8 {
        return Err(FuturesError::InvalidMarketStatus.into());
    }
    if market_state.settlement_price_micro == 0 {
        return Err(FuturesError::PriceNotPosted.into());
    }

    let mut position_state = load_position(&position.data.borrow())?;
    if position_state.owner != *owner.key {
        return Err(FuturesError::UnauthorizedOwner.into());
    }
    if position_state.settled != 0 {
        return Err(FuturesError::PositionAlreadySettled.into());
    }

    let pnl = settlement_pnl_micro(
        position_state.side,
        position_state.contracts,
        market_state.entry_price_micro,
        market_state.settlement_price_micro,
        market_state.contract_mtok,
    )?;
    let fee = settlement_fee_micro(pnl, market_state.fee_bps)?;
    let payout = settlement_payout_micro(position_state.locked_margin, pnl, fee)?;

    let market_id_bytes = market_state.market_id.to_le_bytes();
    let seeds: &[&[u8]] = &[
        b"market",
        market_state.authority.as_ref(),
        market_id_bytes.as_ref(),
        &[market_state.bump],
    ];

    crate::spl_token::transfer(
        token_program,
        vault,
        owner_ata,
        market,
        &[seeds],
        payout,
    )?;

    position_state.settled = 1;
    position_state.contracts = 0;
    position_state.locked_margin = 0;
    position_state.pack_into_slice(&mut position.data.borrow_mut());

    let _ = program_id;
    Ok(())
}

fn load_market(data: &[u8]) -> Result<Market, ProgramError> {
    Market::unpack(data).map_err(|_| FuturesError::UninitializedAccount.into())
}

fn load_market_mut(market: &AccountInfo) -> Result<Market, ProgramError> {
    load_market(&market.data.borrow())
}

fn load_position(data: &[u8]) -> Result<Position, ProgramError> {
    Position::unpack(data).map_err(|_| FuturesError::UninitializedAccount.into())
}
