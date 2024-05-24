use std::fmt;

use crate::common::{self, OrderId, INVALID_ORDER_ID};

#[derive(Clone)]
pub struct OrderInfo {
    pub participant_id: common::ParticipantId,
    pub order_id: common::OrderId,
}

impl Default for OrderInfo {
    fn default() -> Self {
        Self {
            participant_id: common::INVALID_PARTICIPANT_ID,
            order_id: common::INVALID_ORDER_ID,
        }
    }
}

impl fmt::Display for OrderInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Participant ID: {}, Order ID: {}", self.participant_id, self.order_id)
    }
}

pub struct Order {
    pub symbol_id: common::SymbolId,
    pub order_info: OrderInfo,
    pub internal_order_id: common::OrderId,
    pub side: common::Side,
    pub price: common::Price,
    pub qty: common::Quantity,
    pub priority: common::Priority,
    pub prev_order_info: OrderInfo,
    pub next_order_info: OrderInfo,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Order [symb: {}, order: {}, ioid: {}, side: {}, price: {}, qty: {}, priority: {}, prev: {}, next:{}]",
        self.symbol_id,
        self.order_info,
        self.internal_order_id,
        self.side,
        self.price,
        self.qty,
        self.priority,
        self.prev_order_info,
        self.next_order_info)
    }
}

impl Default for Order {
    fn default() -> Self {
        Self {
            symbol_id: common::INVALID_SYMBOL_ID,
            order_info: OrderInfo::default(),
            internal_order_id: common::INVALID_ORDER_ID,
            side: common::Side::Invalid,
            price: common::INVALID_PRICE,
            qty: common::INVALID_QUANTITY,
            priority: common::INVALID_PRIORITY,
            prev_order_info: OrderInfo::default(),
            next_order_info: OrderInfo::default(),
        }
    }
}

pub struct OrderAtPrice {
    pub side: common::Side,
    pub price: common::Price,
    pub head_order_info: OrderInfo,
    pub prev_idx: usize,
    pub next_idx: usize
}

impl Default for OrderAtPrice {
    fn default() -> Self {
        Self {
            side: common::Side::Invalid,
            price: common::INVALID_PRICE,
            head_order_info: OrderInfo::default(),
            prev_idx: common::MAX_PRICE_LEVELS,
            next_idx: common::MAX_PRICE_LEVELS
        }
    }
}

impl fmt::Display for OrderAtPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Order [side: {}, price: {}, head_order: {}, prev_id: {}, next_id: {}]",
        self.side,
        self.price,
        self.head_order_info,   
        self.prev_idx,
        self.next_idx)
    }
}


pub type OrderPtr = Option<refpool::PoolBox<Order>>;
pub type OrderAtPricePtr = Option<refpool::PoolBox<OrderAtPrice>>;
pub type OrderHashMap =  Vec<OrderPtr>;  //MAP ORDER AND IDS
pub type OrderAtPriceLevelHashMap = Vec<OrderAtPricePtr>; //MAP OrderAtPrice With Price
pub type ParticipantOrderHashMap = Vec<OrderHashMap>; // MAP PARTICIPANT AND ORDERS


pub fn create_order_at_price_level_hash_map() -> OrderAtPriceLevelHashMap {
    let mut orders: OrderAtPriceLevelHashMap = Vec::new();
    orders.reserve(common::MAX_PRICE_LEVELS);
    orders
}

pub fn create_order_hash_map() -> OrderHashMap {
    let mut orders: OrderHashMap = Vec::new();
    orders.reserve(common::MAX_ORDER_IDS);
    orders
}

pub fn create_participant_order_hash_map() -> ParticipantOrderHashMap  {
    let mut orders: ParticipantOrderHashMap = Vec::new();
    orders.reserve(common::MAX_PARTICIPANTS_NUMBER);
    orders
}

