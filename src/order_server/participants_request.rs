use std::fmt::{self};

use crate::common::{self, INVALID_ORDER_ID};

#[repr(u8)]
pub enum ParticipantRequestType {
    Invalid = 0,
    New = 1,
    Cancel = 2
}

impl fmt::Display for ParticipantRequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParticipantRequestType::New => write!(f, "NEW"),
            ParticipantRequestType::Cancel => write!(f, "CANCEL"),
            ParticipantRequestType::Invalid => write!(f, "INVALID"),
        }
    }
}

pub struct ParticipantRequest {
    request_type: ParticipantRequestType,
    participant_id: common::ParticipantId,
    symbol_id: common::SymbolId,
    order_id: common::OrderId,
    side: common::Side,
    price: common::Price
}

impl fmt::Display for ParticipantRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParticipantRequest[type: {}, ptid:{}, symb:{}, order:{}, side:{}, price:{}]", self.request_type, self.participant_id, self.symbol_id, self.order_id, self.side, self.price)
    }
}

impl Default for ParticipantRequest {
    fn default() -> Self {
        Self {
            request_type: ParticipantRequestType::Invalid,
            participant_id: common::INVALID_PARTICIPANT_ID,
            symbol_id: common::INVALID_SYMBOL_ID,
            order_id: INVALID_ORDER_ID,
            side: common::Side::Invalid,
            price: common::INVALID_PRICE
        }
    }
}