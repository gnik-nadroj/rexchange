use std::iter;

use refpool::PoolBox;

use crate::{common::{OrderId, ParticipantId, Price, Priority, Quantity, Side, SymbolId, INVALID_ORDER_ID, INVALID_PRICE, INVALID_QUANTITY, MAX_ORDER_IDS, MAX_PRICE_LEVELS}, market_data::market_update::{MarketUpdate, MarketUpdateType}, order_server::participants_response::{ParticipantResponse, ParticipantResponseType}};

use super::{matching_engine::MatchingEngine, order::{self, create_order_at_price_level_hash_map, create_participant_order_hash_map, Order, OrderAtPrice, OrderAtPriceLevelHashMap, OrderAtPricePtr, OrderInfo, OrderPtr, ParticipantOrderHashMap}};

struct OrderBook {
    participants_orders: ParticipantOrderHashMap,
    orders_at_price_level: OrderAtPriceLevelHashMap, 
    order_at_price_level_pool: refpool::Pool<OrderAtPrice>,
    order_pool: refpool::Pool<Order>,
    symbol_id: SymbolId,
    best_bid_idx: usize,
    best_ask_idx: usize,
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
            best_bid_idx: MAX_PRICE_LEVELS,
            best_ask_idx: MAX_PRICE_LEVELS,
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

    fn check_for_match(&self, order_info: OrderInfo, price: Price, qty: Quantity, internal_order_id: OrderId) -> Quantity {
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

    fn add_order_at_price(&mut self, mut new_order_at_price: PoolBox<OrderAtPrice>) {
        let new_order_at_price_index = price_to_index(new_order_at_price.price);

        let (best_order_at_price_index, is_not_better): (&mut _, Box<dyn Fn(&OrderAtPrice, &OrderAtPrice) -> bool>) = match new_order_at_price.side {
            Side::Invalid => panic!("INVALID side aren't taken into account"),
            Side::Buy => {
                let compare = Box::new(|order1: &OrderAtPrice, order2: &OrderAtPrice| order1.price < order2.price);
                (&mut self.best_bid_idx, compare)
            },
            Side::Sell => {
                let compare = Box::new(|order1: &OrderAtPrice, order2: &OrderAtPrice| order1.price > order2.price);
                (&mut self.best_ask_idx, compare)
            },
        };
        

        if *best_order_at_price_index == MAX_PRICE_LEVELS {
            new_order_at_price.next_idx = new_order_at_price_index;
            new_order_at_price.prev_idx = new_order_at_price_index;
            *best_order_at_price_index = new_order_at_price_index;
        } else {
            let mut target_idx = *best_order_at_price_index;
            
            let mut add_after = true;

            let mut is_first_iter = true;

            while add_after && (is_first_iter || target_idx != *best_order_at_price_index) {
                let order_at_price = self.orders_at_price_level[target_idx].as_ref().unwrap();
                add_after = is_not_better(&new_order_at_price, &order_at_price);
                if add_after {
                    target_idx = order_at_price.next_idx;
                }
                is_first_iter = false;  
            }

            if add_after {
                let prev_order_at_price_next_idx: usize;

                {
                    let prev_order_at_price = self.orders_at_price_level[target_idx].as_mut().unwrap();
                    prev_order_at_price_next_idx = prev_order_at_price.next_idx;
                    new_order_at_price.next_idx = prev_order_at_price_next_idx;
                    new_order_at_price.prev_idx = target_idx;
                    prev_order_at_price.next_idx = new_order_at_price_index;
                }

                let next_order_at_price = self.orders_at_price_level[target_idx].as_mut().unwrap();
                next_order_at_price.prev_idx = target_idx;
            } else {
                let prev_order_at_price_prev_idx: usize;
                
                {
                    let next_order_at_price = self.orders_at_price_level[target_idx].as_mut().unwrap();
                    prev_order_at_price_prev_idx = next_order_at_price.prev_idx;
                    new_order_at_price.next_idx = target_idx;
                    new_order_at_price.prev_idx = prev_order_at_price_prev_idx;
                    next_order_at_price.prev_idx = new_order_at_price_index;
                }

                let prev_order_at_price = self.orders_at_price_level[prev_order_at_price_prev_idx].as_mut().unwrap();
                prev_order_at_price.next_idx = target_idx; 
            }

            let best_order_at_price = self.orders_at_price_level[*best_order_at_price_index].as_mut().unwrap();
            
            if !is_not_better(&new_order_at_price, &best_order_at_price) {
                *best_order_at_price_index = new_order_at_price_index;
            }
        }

        
        self.orders_at_price_level[new_order_at_price_index] = Some(new_order_at_price);
    }

    fn add_order(&mut self,  mut order: PoolBox<Order>) {
        let order_at_price_level_opt = self.get_order_at_price(order.price);
        let order_info = OrderInfo{order_id: order.order_info.order_id, participant_id: order.order_info.participant_id};

        if let None = order_at_price_level_opt {
            order.next_order_info = order_info.clone();
            order.prev_order_info = order_info.clone();

            let new_order_at_price = PoolBox::new(&self.order_at_price_level_pool, OrderAtPrice{
                side: order.side.clone(),
                price: order.price,
                head_order_info: order_info.clone(),
                prev_idx: MAX_PRICE_LEVELS,
                next_idx: MAX_PRICE_LEVELS
            });

            self.add_order_at_price(new_order_at_price);
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

    pub fn add(&mut self, order_info: OrderInfo, side: Side, price: Price, qty: Quantity, engine: &mut MatchingEngine) {
        let internal_order_id = self.generate_new_order_id();
        self.participant_response = ParticipantResponse {
            response_type: ParticipantResponseType::Accepted,
            participant_id: order_info.participant_id,
            participant_order_id: order_info.order_id,
            symbol_id: self.symbol_id,
            internal_order_id,
            side: side.clone(),
            price,
            exec_qty: 0,
            leaves_qty: qty
        };

        engine.send_participant_response(&self.participant_response);

        let leaves_qty = self.check_for_match(order_info.clone(), price, qty, internal_order_id);

        if leaves_qty > 0 {
            let priority = self.get_next_priority(price);
            let order = PoolBox::new(&self.order_pool, 
                          Order { 
                                    symbol_id: self.symbol_id, 
                                    order_info: order_info.clone(),
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

    fn remove_order(&mut self, order_info: OrderInfo) {
        
    }

    fn cancel(&mut self, order_info: OrderInfo, engine: &mut MatchingEngine) {
        let is_cancelable = order_info.order_id < MAX_ORDER_IDS.try_into().unwrap() && 
                                  self.participants_orders[order_info.participant_id as usize][order_info.order_id as usize].is_some();

        if is_cancelable {
            {
                let order_to_cancel = self.participants_orders[order_info.participant_id as usize][order_info.order_id as usize].as_mut().unwrap();

                self.participant_response = ParticipantResponse {
                    response_type: ParticipantResponseType::Cancelled,
                    participant_id: order_info.participant_id,
                    symbol_id: self.symbol_id,
                    participant_order_id: order_info.order_id,
                    internal_order_id: order_to_cancel.internal_order_id,
                    side: order_to_cancel.side.clone(),
                    price: order_to_cancel.price,
                    exec_qty: INVALID_QUANTITY,
                    leaves_qty: order_to_cancel.qty
                };
    
                self.market_update = MarketUpdate{
                    update_type: MarketUpdateType::Cancel,
                    order_id: order_to_cancel.internal_order_id,
                    symbol_id: self.symbol_id,
                    side: order_to_cancel.side.clone(),
                    price: order_to_cancel.price,
                    qty: order_to_cancel.qty,
                    priority: order_to_cancel.priority,
                };
    
                engine.send_market_update(&self.market_update);
            }

            self.remove_order(order_info);
        } else {
            self.participant_response = ParticipantResponse {
                response_type: ParticipantResponseType::CancelRejected,
                participant_id: order_info.participant_id,
                symbol_id: self.symbol_id,
                participant_order_id: order_info.order_id,
                internal_order_id: INVALID_ORDER_ID,
                side: Side::Invalid,
                price: INVALID_PRICE,
                exec_qty: INVALID_QUANTITY,
                leaves_qty: INVALID_QUANTITY,
            }
        }


        engine.send_participant_response(&self.participant_response);
    }
}


fn price_to_index(price: Price) -> usize {
    price as usize % MAX_PRICE_LEVELS
}

type OrderbookHashmap = Vec<OrderBook>; //map symbol id with orderbook

