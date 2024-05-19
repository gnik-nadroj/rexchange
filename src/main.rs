use crate::order_server::participants_request;

mod common;
mod order_server;

fn main() {
    let e = common::INVALID_ORDER_ID;

    let c = common::Side::Buy;
    let d = common::Side::Sell;
    let e = common::Side::Invalid;


    let server = participants_request::ParticipantRequest::default();


    println!("{}", server);
}
