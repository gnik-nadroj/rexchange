use std::fmt;

use crate::common;

#[repr(u8)]
pub enum ParticipantResponseType {
    Invalid = 0,
    Accepted,
    Cancelled,
    Filled,
    CancelRejected
}

impl fmt::Display for ParticipantResponseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParticipantResponseType::Accepted => write!(f, "NEW"),
            ParticipantResponseType::Cancelled => write!(f, "CANCELLED"),
            ParticipantResponseType::Filled => write!(f, "FILLED"),
            ParticipantResponseType::CancelRejected => write!(f, "CANCEL-REJECTED"),
            ParticipantResponseType::Invalid => write!(f, "INVALID"),
        }
    }
}
pub struct ParticipantResponse {
    response_type: ParticipantResponseType,
    participant_id: common::ParticipantId,
    symbol_id: common::SymbolId,
    participant_order_id: common::OrderId,
    market_order_id: common::OrderId,
    side: common::Side,
    price: common::Price,
    exec_qty: common::Quantity,
    leaves_qty: common::Quantity,
}


impl fmt::Display for ParticipantResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "ParticipantResponse [type: {}, ptid: {}, symb: {}, coid: {}, moid: {}, side: {}, exec_qty: {}, leaves_qty: {}, price: {}]",
        self.response_type,
        self.participant_id,
        self.symbol_id,
        self.participant_order_id,
        self.market_order_id,
        self.side,
        self.exec_qty,
        self.leaves_qty,
        self.price)
    }
}

impl Default for ParticipantResponse {
    fn default() -> Self {
        Self {
            response_type: ParticipantResponseType::Invalid,
            participant_id: common::INVALID_PARTICIPANT_ID,
            symbol_id: common::INVALID_SYMBOL_ID,
            participant_order_id: common::INVALID_ORDER_ID,
            market_order_id: common::INVALID_ORDER_ID,
            side: common::Side::Invalid,
            exec_qty: common::INVALID_QUANTITY,
            leaves_qty: common::INVALID_QUANTITY,
            price: common::INVALID_PRICE
        }
    }
}