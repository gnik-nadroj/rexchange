use crate::{market_data::market_update, order_server::{participants_request, participants_response}};

mod common;
mod order_server;
mod market_data;
mod matching_engine;

fn main() {
    let e = common::INVALID_ORDER_ID;

    let c = common::Side::Buy;
    let d = common::Side::Sell;
    let e = common::Side::Invalid;


    let req = participants_request::ParticipantRequest::default();
    let res = participants_response::ParticipantResponse::default();
    let market_update = market_update::MarketUpdate::default();

    

    println!("{}", req);
    println!("{}", res);
    println!("{}", market_update);
}
