use std::fmt;

#[repr(u8)]
pub enum Side {
    Invalid = 0,
    Buy = 1,
    Sell = 2,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
            Side::Invalid => write!(f, "INVALID"),
        }
    }
}

pub type OrderId = u64;
pub const INVALID_ORDER_ID: u64 = u64::MAX;

pub type SymbolId = u32;
pub const INVALID_SYMBOL_ID: u32 = u32::MAX;

pub type ParticipantId = u32;
pub const INVALID_PARTICIPANT_ID: u32 = u32::MAX;

pub type Price = u64;
pub const INVALID_PRICE: u64 = u64::MAX;


pub type Quantity = u32;
pub const INVALID_QUANTITY: u32 = u32::MAX;

pub type Priority = u64;
pub const INVALID_PRIORITY: u64 = u64::MAX;

pub const MAX_SYMBOL: u32 = 8;
pub const MAX_PARTICIPANTS_UPDATES: u32 = 256 * 1024;
pub const MAX_MARKET_UPDATES: u32 = 256 * 1024;
pub const MAX_PARTICIPANTS_NUMBER: u32 = 256;
pub const MAX_ORDER_IDS: u32 = 1024 * 1024;
pub const MAX_PRICE_LEVELS: u32 = 256;