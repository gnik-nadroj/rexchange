use std::fmt;

use crate::common;

#[derive(Clone)]
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
    pub response_type: ParticipantResponseType,
    pub participant_id: common::ParticipantId,
    pub symbol_id: common::SymbolId,
    pub participant_order_id: common::OrderId,
    pub internal_order_id: common::OrderId,
    pub side: common::Side,
    pub price: common::Price,
    pub exec_qty: common::Quantity,
    pub leaves_qty: common::Quantity,
}


impl fmt::Display for ParticipantResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "ParticipantResponse [type: {}, ptid: {}, symb: {}, poid: {}, ioid: {}, side: {}, exec_qty: {}, leaves_qty: {}, price: {}]",
        self.response_type,
        self.participant_id,
        self.symbol_id,
        self.participant_order_id,
        self.internal_order_id,
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
            internal_order_id: common::INVALID_ORDER_ID,
            side: common::Side::Invalid,
            exec_qty: common::INVALID_QUANTITY,
            leaves_qty: common::INVALID_QUANTITY,
            price: common::INVALID_PRICE
        }
    }
}