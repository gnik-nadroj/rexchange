use std::fmt;

use crate::common;

pub struct Order {
    pub symbol_id: common::SymbolId,
    pub participant_id: common::ParticipantId,
    pub participant_order_id: common::OrderId,
    pub internal_order_id: common::OrderId,
    pub side: common::Side,
    pub price: common::Price,
    pub qty: common::Quantity,
    pub priority: common::Priority,
    pub prev_order: Option<refpool::PoolRef<Order>>,
    pub next_order: Option<refpool::PoolRef<Order>>
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Order [symb: {}, ptid: {}, poid: {}, ioid: {}, side: {}, price: {}, qty: {}, priority: {}, prev: {}, next:{}]",
        self.symbol_id,
        self.participant_id,
        self.participant_order_id,
        self.internal_order_id,
        self.side,
        self.price,
        self.qty,
        self.priority,
        self.prev_order.as_ref().map_or(common::INVALID_ORDER_ID, |order| order.internal_order_id),
        self.next_order.as_ref().map_or(common::INVALID_ORDER_ID, |order| order.internal_order_id))
    }
}

impl Default for Order {
    fn default() -> Self {
        Self {
            symbol_id: common::INVALID_SYMBOL_ID,
            participant_id: common::INVALID_PARTICIPANT_ID,
            participant_order_id: common::INVALID_ORDER_ID,
            internal_order_id: common::INVALID_ORDER_ID,
            side: common::Side::Invalid,
            price: common::INVALID_PRICE,
            qty: common::INVALID_QUANTITY,
            priority: common::INVALID_PRIORITY,
            prev_order: None,
            next_order: None
        }
    }
}

pub struct OrderAtPrice {
    pub side: common::Side,
    pub price: common::Price,
    pub head_order: Option<refpool::PoolRef<Order>>,
    pub prev_entry: Option<refpool::PoolRef<OrderAtPrice>>,
    pub next_entry: Option<refpool::PoolRef<OrderAtPrice>>
}

impl Default for OrderAtPrice {
    fn default() -> Self {
        Self {
            side: common::Side::Invalid,
            price: common::INVALID_PRICE,
            head_order: None,
            prev_entry: None,
            next_entry: None
        }
    }
}

impl fmt::Display for OrderAtPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Order [side: {}, price: {}, head_order: {}, prev: {}, next: {}]",
        self.side,
        self.price,
        self.head_order.as_ref().map_or("null".to_string(), |order| order.to_string()),       
        self.prev_entry.as_ref().map_or(common::INVALID_ORDER_ID, |order_at_price| order_at_price.price),
        self.next_entry.as_ref().map_or(common::INVALID_ORDER_ID, |order_at_price| order_at_price.price))
    }
}

pub type OrderAtPriceHashMap = [OrderAtPrice; common::MAX_PRICE_LEVELS as usize];
