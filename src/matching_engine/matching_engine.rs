use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::market_data::market_update::MarketUpdate;
use crate::order_server::participants_request::ParticipantRequest;
use crate::order_server::participants_response::ParticipantResponse;

pub struct MatchingEngine {
    participants_requests: Receiver<ParticipantRequest>,
    participants_response: Sender<ParticipantResponse>,
    market_data_updates: Sender<MarketUpdate>
}


impl MatchingEngine {
    pub fn run() {

    }

    pub fn send_participant_response(&mut self, response: &ParticipantResponse) {

    }

    pub fn send_market_update(&mut self, update: &MarketUpdate) {
        
    }
}
