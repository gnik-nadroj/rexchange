use std::{fmt, thread};

#[derive(Clone)]
#[repr(u8)]
pub enum Side {
    Invalid = 0,
    Buy,
    Sell,
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

pub const MAX_SYMBOL: usize = 8;
pub const MAX_PARTICIPANTS_UPDATES: usize = 256 * 1024;
pub const MAX_MARKET_UPDATES: usize = 256 * 1024;
pub const MAX_PARTICIPANTS_NUMBER: usize = 256;
pub const MAX_ORDER_IDS: usize = 1024 * 1024;
pub const MAX_PRICE_LEVELS: usize = 256;

fn spawn_pinned<F>(f: F, core_id: isize) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(move || {
        if core_id >= 0 {
            let core_ids = core_affinity::get_core_ids().unwrap();
            if core_id as usize >= core_ids.len() {
                panic!("Invalid core_id: {}", core_id);
            }
            core_affinity::set_for_current(core_ids[core_id as usize]);
        }
        f();
    })
}