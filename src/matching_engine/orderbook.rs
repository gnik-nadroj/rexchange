use refpool::PoolBox;

use crate::{common::{OrderId, ParticipantId, Price, Priority, Quantity, Side, SymbolId, INVALID_ORDER_ID, MAX_ORDER_IDS, MAX_PRICE_LEVELS}, market_data::market_update::{MarketUpdate, MarketUpdateType}, order_server::participants_response::{ParticipantResponse, ParticipantResponseType}};

use super::{matching_engine::MatchingEngine, order::{self, create_order_at_price_level_hash_map, create_participant_order_hash_map, Order, OrderAtPrice, OrderAtPriceLevelHashMap, OrderAtPricePtr, OrderInfo, OrderPtr, ParticipantOrderHashMap}};



struct OrderBook {
    participants_orders: ParticipantOrderHashMap,
    orders_at_price_level: OrderAtPriceLevelHashMap, 
    order_at_price_level_pool: refpool::Pool<OrderAtPrice>,
    order_pool: refpool::Pool<Order>,
    symbol_id: SymbolId,
    bids: OrderAtPricePtr,
    asks: OrderAtPricePtr,
    next_internal_order_id: OrderId,
    participant_response: ParticipantResponse,
    market_update: MarketUpdate,
}


impl OrderBook {
    pub fn new(symbol_id: SymbolId) -> Self {
        Self {
            participants_orders: create_participant_order_hash_map(),
            orders_at_price_level: create_order_at_price_level_hash_map(),
            order_at_price_level_pool: refpool::Pool::new(MAX_PRICE_LEVELS as usize),
            order_pool: refpool::Pool::new(MAX_ORDER_IDS as usize),
            symbol_id,
            bids: None,
            asks: None,
            next_internal_order_id: 1,
            participant_response: ParticipantResponse::default(),
            market_update: MarketUpdate::default()
        }
    }

    fn generate_new_order_id(&mut self) -> OrderId {
        let id = self.next_internal_order_id;
        self.next_internal_order_id += 1;
        id
    }

    fn get_order_at_price(&self, price: Price) -> &OrderAtPricePtr {
        &self.orders_at_price_level[price_to_index(price)]
    }

    fn get_participant_order(&self, order_info: OrderInfo) -> &OrderPtr {
        &self.participants_orders[order_info.participant_id as usize][order_info.order_id as usize]
    }

    fn check_for_match(&self, participant_id: ParticipantId, participant_order_id: OrderId, price: Price, qty: Quantity, internal_order_id: OrderId) -> Quantity {
        return 0;
    }

    fn get_next_priority(&self, price: Price) -> Priority {
        let order_at_price_level_opt = self.get_order_at_price(price);

        if let Some(order_at_price_level) = order_at_price_level_opt {
            let head_order_info = order_at_price_level.head_order_info.clone();
            let head_order = self.get_participant_order(head_order_info).as_ref().unwrap();

            let last_order_info = head_order.prev_order_info.clone();
            let last_order = self.get_participant_order(last_order_info).as_ref().unwrap();

            return last_order.priority + 1;
        }

        1 
    }

    fn add_order_at_price(&mut self, order_at_price: &mut PoolBox<OrderAtPrice>) {
        
    }

    fn add_order(&mut self,  mut order: PoolBox<Order>) {
        let order_at_price_level_opt = self.get_order_at_price(order.price);
        let order_info = OrderInfo{order_id: order.order_info.order_id, participant_id: order.order_info.participant_id};

        if let None = order_at_price_level_opt {
            order.next_order_info = order_info.clone();
            order.prev_order_info = order_info.clone();

            let mut new_order_at_price = PoolBox::new(&self.order_at_price_level_pool, OrderAtPrice{
                side: order.side.clone(),
                price: order.price,
                head_order_info: order_info.clone(),
                prev_id: MAX_PRICE_LEVELS,
                next_id: MAX_PRICE_LEVELS
            });

            self.add_order_at_price(&mut new_order_at_price);
        } else {
            let head_order_info = order_at_price_level_opt.as_ref().unwrap().head_order_info.clone();
            let last_order_info: OrderInfo;

            {
                let head_order = self.get_participant_order(head_order_info.clone()).as_ref().unwrap();
                last_order_info = head_order.prev_order_info.clone();
            }

            {
                order.prev_order_info = last_order_info.clone();
                order.next_order_info = head_order_info.clone();
                let last_order = self.participants_orders[last_order_info.participant_id as usize][last_order_info.order_id as usize].as_mut().unwrap();
                last_order.next_order_info = order_info.clone();
            }

            {
                let head_order = self.participants_orders[head_order_info.participant_id as usize][head_order_info.order_id as usize].as_mut().unwrap();
                head_order.prev_order_info = order_info.clone();
            } 
        }

        self.participants_orders[order_info.participant_id as usize][order_info.order_id as usize] = Some(order);
    }

    pub fn add(&mut self, participant_id: ParticipantId, participant_order_id: OrderId, side: Side, price: Price, qty: Quantity, engine: &mut MatchingEngine) {
        let internal_order_id = self.generate_new_order_id();
        self.participant_response = ParticipantResponse {
            response_type: ParticipantResponseType::Accepted,
            participant_id,
            participant_order_id,
            symbol_id: self.symbol_id,
            internal_order_id,
            side: side.clone(),
            price,
            exec_qty: 0,
            leaves_qty: qty
        };

        engine.send_participant_response(&self.participant_response);

        let leaves_qty = self.check_for_match(participant_id, participant_order_id, price, qty, internal_order_id);

        if leaves_qty > 0 {
            let priority = self.get_next_priority(price);
            let mut order = PoolBox::new(&self.order_pool, 
                          Order { 
                                    symbol_id: self.symbol_id, 
                                    order_info: OrderInfo { participant_id, order_id: participant_order_id },
                                    internal_order_id, 
                                    side: side.clone(), 
                                    price, 
                                    qty, 
                                    priority, 
                                    prev_order_info: OrderInfo::default(), 
                                    next_order_info: OrderInfo::default() });

            self.add_order(order);

            let market_update = MarketUpdate{
                update_type: MarketUpdateType::Add,
                order_id: internal_order_id,
                symbol_id: self.symbol_id,
                side: side.clone(),
                price,
                priority,
                qty
            };
            
            engine.send_market_update(&market_update);
        }
    }
}


fn price_to_index(price: Price) -> usize {
    price as usize % MAX_PRICE_LEVELS
}

type OrderbookHashmap = Vec<OrderBook>; //map symbol id with orderbook

