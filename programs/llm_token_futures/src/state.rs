use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const MARKET_MAGIC: u64 = 0x4D4B545F_4C544631; // "MKTF_LTF1"
pub const POSITION_MAGIC: u64 = 0x504F535F_4C544631; // "POS_LTF1"

pub const MARKET_LEN: usize = 8 + 197;
pub const POSITION_LEN: usize = 8 + 90;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MarketStatus {
    Open = 0,
    Halted = 1,
    PricePosted = 2,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Side {
    Long = 0,
    Short = 1,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Market {
    pub magic: u64,
    pub authority: Pubkey,
    pub oracle: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub market_id: u64,
    pub entry_price_micro: u64,
    pub max_settlement_price_micro: u64,
    pub settlement_price_micro: u64,
    pub open_ts: i64,
    pub trade_cutoff_ts: i64,
    pub expiry_ts: i64,
    pub fee_bps: u16,
    pub contract_mtok: u64,
    pub status: u8,
    pub bump: u8,
    pub vault_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Position {
    pub magic: u64,
    pub owner: Pubkey,
    pub market: Pubkey,
    pub side: u8,
    pub contracts: u64,
    pub locked_margin: u64,
    pub settled: u8,
    pub bump: u8,
}

impl Market {
    pub fn unpack(data: &[u8]) -> Result<Self, std::io::Error> {
        Market::try_from_slice(data)
    }

    pub fn pack_into_slice(&self, dst: &mut [u8]) {
        self.serialize(&mut &mut dst[..]).expect("market pack");
    }
}

impl Position {
    pub fn unpack(data: &[u8]) -> Result<Self, std::io::Error> {
        Position::try_from_slice(data)
    }

    pub fn pack_into_slice(&self, dst: &mut [u8]) {
        self.serialize(&mut &mut dst[..]).expect("position pack");
    }

    pub fn is_initialized(&self) -> bool {
        self.magic == POSITION_MAGIC
    }
}
