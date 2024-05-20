use std::fmt::{self};

use crate::common;

#[repr(u8)]
pub enum MarketUpdateType {
    Invalid = 0,
    Add,
    Modify,
    Cancel,
    Trade
}

impl fmt::Display for MarketUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarketUpdateType::Add => write!(f, "ADD"),
            MarketUpdateType::Modify => write!(f, "MODIFY"),
            MarketUpdateType::Cancel => write!(f, "CANCEL"),
            MarketUpdateType::Trade => write!(f, "TRADE"),
            MarketUpdateType::Invalid => write!(f, "INVALID"),
        }
    }
}

/*MarketUpdateType type_ = MarketUpdateType::INVALID;
    OrderId order_id_ = OrderId_INVALID;
    TickerId ticker_id_ = TickerId_INVALID;
    Side side_ = Side::INVALID;
    Price price_ = Price_INVALID;
    Qty qty_ = Qty_INVALID;
    Priority priority_ = Priority_INVALID; */
pub struct MarketUpdate {
    update_type: MarketUpdateType,
    order_id: common::OrderId,
    symbol_id: common::SymbolId,
    side: common::Side,
    price: common::Price,
    qty: common::Quantity,
    priority: common::Priority
}

impl fmt::Display for MarketUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MarketUpdate [type: {}, order:{}, symb:{}, side:{}, price:{}, qty:{}, prio:{}]", 
        self.update_type, self.order_id, self.symbol_id, self.side, self.price, self.qty, self.priority)
    }
}

impl Default for MarketUpdate {
    fn default() -> Self {
        Self {
            update_type: MarketUpdateType::Invalid,
            order_id: common::INVALID_ORDER_ID,
            symbol_id: common::INVALID_SYMBOL_ID,
            side: common::Side::Invalid,
            price: common::INVALID_PRICE,
            qty: common::INVALID_QUANTITY,
            priority: common::INVALID_PRIORITY
        }
    }
}