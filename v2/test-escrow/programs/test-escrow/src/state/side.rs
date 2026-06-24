use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum Side {
    Long,
    Short,
}

impl Side {
    pub fn as_seed_byte(self) -> u8 {
        match self {
            Side::Long => 0,
            Side::Short => 1,
        }
    }
}
