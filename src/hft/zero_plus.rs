use crate::hft::market_data::{MarketSnapshot, OrderSide};

/// 0+ HFT Strategy State
#[derive(Debug, Clone)]
pub struct ZeroPlusStrategy {
    pub position: i32,           // Current position (positive = long, negative = short)
    pub last_fill_price: u32,    // Price of last fill
    pub last_fill_side: Option<OrderSide>,
    pub pending_orders: Vec<PendingOrder>,
    pub total_pnl: i64,          // Total P&L in ticks
    pub trades_today: u32,       // Number of trades executed
    pub scratches_today: u32,    // Number of scratches executed
    pub win_rate: f64,           // Winning trade percentage
    pub sharpe_ratio: f64,       // Current Sharpe ratio estimate
}

#[derive(Debug, Clone)]
pub struct PendingOrder {
    pub order_id: u64,
    pub price: u32,
    pub quantity: u32,
    pub side: OrderSide,
    pub timestamp: u64,
    pub can_scratch: bool,       // Can this order be scratched if needed?
}

#[derive(Debug, Clone)]
pub struct TradingSignal {
    pub action: TradingAction,
    pub price: u32,
    pub quantity: u32,
    pub urgency: SignalUrgency,
}

#[derive(Debug, Clone)]
pub enum TradingAction {
    Buy,
    Sell,
    Scratch,     // Cancel previous trade
    Hold,        // No action
    Cancel(u64), // Cancel specific order
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignalUrgency {
    Immediate,   // Must execute within 1 microsecond
    Fast,        // Must execute within 10 microseconds  
    Normal,      // Can execute within 100 microseconds
}

impl ZeroPlusStrategy {
    pub fn new() -> Self {
        Self {
            position: 0,
            last_fill_price: 0,
            last_fill_side: None,
            pending_orders: Vec::new(),
            total_pnl: 0,
            trades_today: 0,
            scratches_today: 0,
            win_rate: 0.0,
            sharpe_ratio: 0.0,
        }
    }

    /// Core 0+ strategy logic - processes market data and generates trading signals
    pub fn process_market_data(&mut self, snapshot: &MarketSnapshot) -> TradingSignal {
        // Step 1: Check if we need to scratch any existing positions
        if self.should_scratch(snapshot) {
            return self.generate_scratch_signal(snapshot);
        }

        // Step 2: Look for strong queue opportunities
        let signal = self.find_queue_opportunity(snapshot);
        
        // Step 3: Update strategy state
        self.update_performance_metrics();

        signal
    }

    /// Determine if we should scratch (cancel) current position
    fn should_scratch(&self, snapshot: &MarketSnapshot) -> bool {
        if self.position == 0 {
            return false;
        }

        // Scratch if we're in a weak queue and price is moving against us
        match self.last_fill_side.as_ref() {
            Some(OrderSide::Buy) => {
                // We bought, scratch if bid queue becomes weak or price drops
                !snapshot.bid_queue_strength || 
                snapshot.best_bid_price < self.last_fill_price
            }
            Some(OrderSide::Sell) => {
                // We sold, scratch if ask queue becomes weak or price rises  
                !snapshot.ask_queue_strength ||
                snapshot.best_ask_price > self.last_fill_price
            }
            None => false,
        }
    }

    /// Generate a scratch signal to cancel current position
    fn generate_scratch_signal(&mut self, snapshot: &MarketSnapshot) -> TradingSignal {
        let action = match self.last_fill_side.as_ref() {
            Some(OrderSide::Buy) => TradingAction::Sell,
            Some(OrderSide::Sell) => TradingAction::Buy,
            None => TradingAction::Hold,
        };

        let price = match action {
            TradingAction::Sell => snapshot.best_bid_price,
            TradingAction::Buy => snapshot.best_ask_price,
            _ => 0,
        };

        self.scratches_today += 1;
        
        TradingSignal {
            action,
            price,
            quantity: self.position.abs() as u32,
            urgency: SignalUrgency::Immediate,
        }
    }

    /// Find strong queue opportunities for new trades
    fn find_queue_opportunity(&self, snapshot: &MarketSnapshot) -> TradingSignal {
        // Only trade if we're flat (no position)
        if self.position != 0 {
            return TradingSignal {
                action: TradingAction::Hold,
                price: 0,
                quantity: 0,
                urgency: SignalUrgency::Normal,
            };
        }

        // Only trade if spread is exactly 1 tick (optimal for 0+ strategy)
        if snapshot.spread != 1 {
            return TradingSignal {
                action: TradingAction::Hold,
                price: 0,
                quantity: 0,
                urgency: SignalUrgency::Normal,
            };
        }

        // Look for strong bid queue to join
        if snapshot.bid_queue_strength && snapshot.best_bid_qty >= 100 {
            return TradingSignal {
                action: TradingAction::Buy,
                price: snapshot.best_bid_price,
                quantity: 50, // Conservative size
                urgency: SignalUrgency::Fast,
            };
        }

        // Look for strong ask queue to join  
        if snapshot.ask_queue_strength && snapshot.best_ask_qty >= 100 {
            return TradingSignal {
                action: TradingAction::Sell,
                price: snapshot.best_ask_price,
                quantity: 50, // Conservative size
                urgency: SignalUrgency::Fast,
            };
        }

        // No good opportunities
        TradingSignal {
            action: TradingAction::Hold,
            price: 0,
            quantity: 0,
            urgency: SignalUrgency::Normal,
        }
    }

    /// Update position and P&L after a fill
    pub fn handle_fill(&mut self, price: u32, quantity: u32, side: OrderSide) {
        let signed_quantity = match side {
            OrderSide::Buy => quantity as i32,
            OrderSide::Sell => -(quantity as i32),
        };

        // Calculate P&L if closing a position
        if self.position != 0 && 
           ((self.position > 0 && side == OrderSide::Sell) || 
            (self.position < 0 && side == OrderSide::Buy)) {
            
            let pnl = match side {
                OrderSide::Sell => (price as i64 - self.last_fill_price as i64) * quantity as i64,
                OrderSide::Buy => (self.last_fill_price as i64 - price as i64) * quantity as i64,
            };
            
            self.total_pnl += pnl;
        }

        self.position += signed_quantity;
        self.last_fill_price = price;
        self.last_fill_side = Some(side);
        self.trades_today += 1;
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self) {
        if self.trades_today > 0 {
            // Calculate win rate (scratches count as wins since they avoid losses)
            let total_outcomes = self.trades_today + self.scratches_today;
            let wins = self.scratches_today + (self.total_pnl.max(0) as u32);
            self.win_rate = wins as f64 / total_outcomes as f64;

            // Estimate Sharpe ratio (simplified)
            if self.trades_today >= 10 {
                let avg_pnl = self.total_pnl as f64 / self.trades_today as f64;
                let volatility = 1.0; // Simplified - would need historical data
                self.sharpe_ratio = avg_pnl / volatility;
            }
        }
    }

    /// Get current strategy statistics
    pub fn get_stats(&self) -> StrategyStats {
        StrategyStats {
            total_trades: self.trades_today,
            total_scratches: self.scratches_today,
            current_position: self.position,
            total_pnl_ticks: self.total_pnl,
            total_pnl_dollars: self.total_pnl as f64 * 0.01, // Assuming $0.01 tick size
            win_rate: self.win_rate,
            sharpe_ratio: self.sharpe_ratio,
            scratch_rate: if self.trades_today > 0 { 
                self.scratches_today as f64 / self.trades_today as f64 
            } else { 
                0.0 
            },
        }
    }
}

#[derive(Debug)]
pub struct StrategyStats {
    pub total_trades: u32,
    pub total_scratches: u32,
    pub current_position: i32,
    pub total_pnl_ticks: i64,
    pub total_pnl_dollars: f64,
    pub win_rate: f64,
    pub sharpe_ratio: f64,
    pub scratch_rate: f64,
}

impl StrategyStats {
    pub fn print(&self) {
        println!("\n=== 0+ STRATEGY STATISTICS ===");
        println!("Total Trades: {}", self.total_trades);
        println!("Total Scratches: {}", self.total_scratches);
        println!("Current Position: {}", self.current_position);
        println!("Total P&L: {} ticks (${:.2})", self.total_pnl_ticks, self.total_pnl_dollars);
        println!("Win Rate: {:.1}%", self.win_rate * 100.0);
        println!("Scratch Rate: {:.1}%", self.scratch_rate * 100.0);
        println!("Sharpe Ratio: {:.2}", self.sharpe_ratio);
    }
}

/// FPGA-optimized decision logic for ultra-low latency
/// This represents the core logic that would be implemented in Verilog
pub fn fpga_trading_decision(
    // Market data inputs (32-bit for FPGA efficiency)
    best_bid_price: u32,
    best_ask_price: u32,
    best_bid_qty: u32,
    best_ask_qty: u32,
    bid_queue_strong: bool,
    ask_queue_strong: bool,
    
    // Strategy state inputs
    current_position: i32,
    last_fill_price: u32,
    last_fill_side: u8, // 0 = None, 1 = Buy, 2 = Sell
    
) -> (u8, u32, u32) { // Returns: (action, price, quantity)
    // Action codes: 0 = Hold, 1 = Buy, 2 = Sell, 3 = Scratch
    
    let spread = if best_ask_price > best_bid_price {
        best_ask_price - best_bid_price
    } else {
        0
    };

    // Check if we need to scratch first
    if current_position != 0 {
        let should_scratch = match last_fill_side {
            1 => !bid_queue_strong || best_bid_price < last_fill_price, // Was buy
            2 => !ask_queue_strong || best_ask_price > last_fill_price, // Was sell
            _ => false,
        };

        if should_scratch {
            let scratch_action = if last_fill_side == 1 { 2 } else { 1 }; // Opposite side
            let scratch_price = if scratch_action == 2 { best_bid_price } else { best_ask_price };
            return (scratch_action, scratch_price, current_position.abs() as u32);
        }
    }

    // Only trade if flat and spread is 1 tick
    if current_position == 0 && spread == 1 {
        // Look for strong bid queue opportunity
        if bid_queue_strong && best_bid_qty >= 100 {
            return (1, best_bid_price, 50); // Buy
        }
        
        // Look for strong ask queue opportunity
        if ask_queue_strong && best_ask_qty >= 100 {
            return (2, best_ask_price, 50); // Sell
        }
    }

    (0, 0, 0) // Hold
}
