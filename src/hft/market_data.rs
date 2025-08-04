use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single order in the order book
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: u32,        // Price in ticks (e.g., $8.03 = 803 ticks for $0.01 tick size)
    pub quantity: u32,
    pub side: OrderSide,
    pub timestamp: u64,    // Microseconds since epoch
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order queue at a specific price level
#[derive(Debug, Clone)]
pub struct OrderQueue {
    pub price: u32,
    pub orders: VecDeque<Order>,
    pub total_quantity: u32,
}

impl OrderQueue {
    pub fn new(price: u32) -> Self {
        Self {
            price,
            orders: VecDeque::new(),
            total_quantity: 0,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.total_quantity += order.quantity;
        self.orders.push_back(order);
    }

    pub fn remove_front(&mut self) -> Option<Order> {
        if let Some(order) = self.orders.pop_front() {
            self.total_quantity = self.total_quantity.saturating_sub(order.quantity);
            Some(order)
        } else {
            None
        }
    }

    pub fn is_strong(&self) -> bool {
        // A queue is considered "strong" if it has >= 3 orders and total quantity >= 100
        self.orders.len() >= 3 && self.total_quantity >= 100
    }

    pub fn is_weak(&self) -> bool {
        // A queue is considered "weak" if it has <= 1 order or total quantity <= 30
        self.orders.len() <= 1 || self.total_quantity <= 30
    }

    pub fn queue_position(&self, order_id: u64) -> Option<usize> {
        self.orders.iter().position(|order| order.id == order_id)
    }
}

/// Market data generator for HFT simulation
pub struct MarketDataSimulator {
    pub current_price: u32,     // Current mid price in ticks
    pub tick_size: u32,         // Tick size (typically 1 for $0.01)
    pub bid_queues: Vec<OrderQueue>,  // Bid queues (buy orders)
    pub ask_queues: Vec<OrderQueue>,  // Ask queues (sell orders)
    pub next_order_id: u64,
    pub current_time: u64,      // Microseconds since epoch
}

impl MarketDataSimulator {
    pub fn new(initial_price: u32) -> Self {
        let mut simulator = Self {
            current_price: initial_price,
            tick_size: 1,
            bid_queues: Vec::new(),
            ask_queues: Vec::new(),
            next_order_id: 1,
            current_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        };

        // Initialize order book with some depth
        simulator.initialize_order_book();
        simulator
    }

    fn initialize_order_book(&mut self) {
        // Create initial bid and ask levels with 1-tick spread for 0+ strategy
        for i in 1..=10 {
            let bid_price = self.current_price - i;
            // Make the ask only 1 tick away from best bid (current_price - 1 + 1 = current_price)
            let ask_price = if i == 1 {
                self.current_price  // Best ask = current_price, best bid = current_price - 1
            } else {
                self.current_price + i - 1  // Maintain normal spacing for other levels
            };

            // Create bid queue
            let mut bid_queue = OrderQueue::new(bid_price);
            for j in 0..3 {
                let order = Order {
                    id: self.next_order_id,
                    price: bid_price,
                    quantity: 50 + j * 25,
                    side: OrderSide::Buy,
                    timestamp: self.current_time,
                };
                bid_queue.add_order(order);
                self.next_order_id += 1;
            }
            self.bid_queues.push(bid_queue);

            // Create ask queue
            let mut ask_queue = OrderQueue::new(ask_price);
            for j in 0..3 {
                let order = Order {
                    id: self.next_order_id,
                    price: ask_price,
                    quantity: 50 + j * 25,
                    side: OrderSide::Sell,
                    timestamp: self.current_time,
                };
                ask_queue.add_order(order);
                self.next_order_id += 1;
            }
            self.ask_queues.push(ask_queue);
        }

        // Sort queues by price
        self.bid_queues.sort_by(|a, b| b.price.cmp(&a.price)); // Descending for bids
        self.ask_queues.sort_by(|a, b| a.price.cmp(&b.price)); // Ascending for asks
    }

    pub fn advance_time(&mut self, microseconds: u64) {
        self.current_time += microseconds;
    }

    pub fn get_best_bid(&self) -> Option<&OrderQueue> {
        self.bid_queues.first()
    }

    pub fn get_best_ask(&self) -> Option<&OrderQueue> {
        self.ask_queues.first()
    }

    pub fn get_spread(&self) -> Option<u32> {
        match (self.get_best_bid(), self.get_best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// Add a new order to the appropriate queue
    pub fn add_order(&mut self, price: u32, quantity: u32, side: OrderSide) -> u64 {
        let order = Order {
            id: self.next_order_id,
            price,
            quantity,
            side: side.clone(),
            timestamp: self.current_time,
        };

        self.next_order_id += 1;

        match side {
            OrderSide::Buy => {
                if let Some(queue) = self.bid_queues.iter_mut().find(|q| q.price == price) {
                    queue.add_order(order);
                } else {
                    let mut new_queue = OrderQueue::new(price);
                    new_queue.add_order(order);
                    self.bid_queues.push(new_queue);
                    self.bid_queues.sort_by(|a, b| b.price.cmp(&a.price));
                }
            }
            OrderSide::Sell => {
                if let Some(queue) = self.ask_queues.iter_mut().find(|q| q.price == price) {
                    queue.add_order(order);
                } else {
                    let mut new_queue = OrderQueue::new(price);
                    new_queue.add_order(order);
                    self.ask_queues.push(new_queue);
                    self.ask_queues.sort_by(|a, b| a.price.cmp(&b.price));
                }
            }
        }

        self.next_order_id - 1
    }

    /// Simulate random market activity
    pub fn simulate_tick(&mut self) {
        self.advance_time(100); // 100 microseconds per tick

        // Simple random number generation for simulation
        let action = (self.current_time.wrapping_mul(1664525).wrapping_add(1013904223)) % 10;

        match action {
            0..=3 => self.add_random_order(),
            4..=6 => self.cancel_random_order(),
            7..=8 => self.execute_market_order(),
            _ => {} // No action
        }
    }

    fn add_random_order(&mut self) {
        let side_rand = (self.current_time.wrapping_mul(1103515245).wrapping_add(12345)) % 2;
        let side = if side_rand == 0 { OrderSide::Buy } else { OrderSide::Sell };
        
        let price_offset = (self.current_time.wrapping_mul(214013).wrapping_add(2531011)) % 5;
        let base_price = match side {
            OrderSide::Buy => self.current_price - 1 - (price_offset as u32),
            OrderSide::Sell => self.current_price + 1 + (price_offset as u32),
        };
        
        let quantity = 25 + ((self.current_time.wrapping_mul(134775813).wrapping_add(1)) % 100) as u32;
        
        self.add_order(base_price, quantity, side);
    }

    fn cancel_random_order(&mut self) {
        let side_rand = (self.current_time.wrapping_mul(1664525).wrapping_add(1013904223)) % 2;
        
        // Randomly cancel an order from a queue
        if side_rand == 0 && !self.bid_queues.is_empty() {
            let queue_idx = ((self.current_time.wrapping_mul(214013).wrapping_add(2531011)) % self.bid_queues.len() as u64) as usize;
            if !self.bid_queues[queue_idx].orders.is_empty() {
                if let Some(order) = self.bid_queues[queue_idx].remove_front() {
                    self.bid_queues[queue_idx].total_quantity = self.bid_queues[queue_idx].total_quantity.saturating_sub(order.quantity);
                }
            }
        } else if !self.ask_queues.is_empty() {
            let queue_idx = ((self.current_time.wrapping_mul(214013).wrapping_add(2531011)) % self.ask_queues.len() as u64) as usize;
            if !self.ask_queues[queue_idx].orders.is_empty() {
                if let Some(order) = self.ask_queues[queue_idx].remove_front() {
                    self.ask_queues[queue_idx].total_quantity = self.ask_queues[queue_idx].total_quantity.saturating_sub(order.quantity);
                }
            }
        }
    }

    fn execute_market_order(&mut self) {
        let side_rand = (self.current_time.wrapping_mul(1103515245).wrapping_add(12345)) % 2;
        
        // Execute a market order that hits the best bid/ask
        if side_rand == 0 {
            // Market sell order hits best bid
            if let Some(best_bid) = self.bid_queues.first_mut() {
                if !best_bid.orders.is_empty() {
                    if let Some(order) = best_bid.remove_front() {
                        best_bid.total_quantity = best_bid.total_quantity.saturating_sub(order.quantity);
                    }
                }
            }
        } else {
            // Market buy order hits best ask
            if let Some(best_ask) = self.ask_queues.first_mut() {
                if !best_ask.orders.is_empty() {
                    if let Some(order) = best_ask.remove_front() {
                        best_ask.total_quantity = best_ask.total_quantity.saturating_sub(order.quantity);
                    }
                }
            }
        }
    }

    /// Get market data snapshot for HFT strategy
    pub fn get_market_snapshot(&self) -> MarketSnapshot {
        MarketSnapshot {
            timestamp: self.current_time,
            best_bid_price: self.get_best_bid().map(|q| q.price).unwrap_or(0),
            best_ask_price: self.get_best_ask().map(|q| q.price).unwrap_or(0),
            best_bid_qty: self.get_best_bid().map(|q| q.total_quantity).unwrap_or(0),
            best_ask_qty: self.get_best_ask().map(|q| q.total_quantity).unwrap_or(0),
            bid_queue_strength: self.get_best_bid().map(|q| q.is_strong()).unwrap_or(false),
            ask_queue_strength: self.get_best_ask().map(|q| q.is_strong()).unwrap_or(false),
            spread: self.get_spread().unwrap_or(0),
        }
    }

    /// Print current order book state
    pub fn print_order_book(&self) {
        println!("\n=== ORDER BOOK ===");
        println!("Time: {} μs", self.current_time);
        
        println!("\nASKS (Sell Orders):");
        for (_i, queue) in self.ask_queues.iter().take(5).enumerate() {
            let strength = if queue.is_strong() { "STRONG" } else if queue.is_weak() { "WEAK" } else { "MEDIUM" };
            println!("  ${:.2} | Qty: {:3} | Orders: {} | {}", 
                queue.price as f64 / 100.0, queue.total_quantity, queue.orders.len(), strength);
        }
        
        if let Some(spread) = self.get_spread() {
            println!("  --- SPREAD: ${:.2} ---", spread as f64 / 100.0);
        }
        
        println!("BIDS (Buy Orders):");
        for (_i, queue) in self.bid_queues.iter().take(5).enumerate() {
            let strength = if queue.is_strong() { "STRONG" } else if queue.is_weak() { "WEAK" } else { "MEDIUM" };
            println!("  ${:.2} | Qty: {:3} | Orders: {} | {}", 
                queue.price as f64 / 100.0, queue.total_quantity, queue.orders.len(), strength);
        }
    }
}

/// Market data snapshot for HFT processing
#[derive(Debug, Clone)]
pub struct MarketSnapshot {
    pub timestamp: u64,
    pub best_bid_price: u32,
    pub best_ask_price: u32,
    pub best_bid_qty: u32,
    pub best_ask_qty: u32,
    pub bid_queue_strength: bool,
    pub ask_queue_strength: bool,
    pub spread: u32,
}
