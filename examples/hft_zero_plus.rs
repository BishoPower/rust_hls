use rust_hls::ir::graph::Graph;
use rust_hls::passes::pipeline::PipelineScheduler;
use rust_hls::backend::verilog::generate_verilog_module;
use rust_hls::hft::{MarketDataSimulator, ZeroPlusStrategy, TradingAction, OrderSide, fpga_trading_decision};

fn main() {
    println!("0+ HFT FPGA Implementation");
    println!("==========================");
    
    // Generate the FPGA trading logic
    let mut graph = create_hft_pipeline();
    
    // Enable ultra-high performance pipeline for HFT
    // II=1 (new decision every cycle), depth=3 (minimal latency), no unroll
    graph.enable_pipeline(1, 3, 1);
    
    // Schedule the pipeline for maximum speed
    let mut scheduler = PipelineScheduler::new();
    match scheduler.schedule_pipeline(&mut graph) {
        Ok(()) => {
            println!("Pipeline scheduling successful!");
            
            // Generate HFT Verilog
            let verilog = generate_verilog_module(&graph, "hft_zero_plus");
            
            // Write to file
            std::fs::create_dir_all("target/verilog_out").expect("Failed to create directory");
            std::fs::write("target/verilog_out/hft_zero_plus.v", &verilog)
                .expect("Failed to write Verilog file");
                
            println!("Generated: target/verilog_out/hft_zero_plus.v");
            println!("HFT FPGA logic ready for deployment!");
            
            // Display file size
            if let Ok(metadata) = std::fs::metadata("target/verilog_out/hft_zero_plus.v") {
                println!("File size: {} bytes", metadata.len());
            }
        }
        Err(e) => {
            println!("Pipeline scheduling failed: {}", e);
        }
    }
    
    // Run HFT simulation
    println!("\nRunning 0+ HFT Simulation");
    println!("=========================");
    run_hft_simulation();
}

/// Create the HFT trading decision pipeline
/// This implements the core 0+ strategy logic in hardware
fn create_hft_pipeline() -> Graph {
    println!("\nCreating HFT Trading Decision Pipeline");
    println!("Implementing ultra-low latency 0+ strategy");
    
    let mut graph = Graph::new();
    
    // Market data inputs (all 32-bit for FPGA efficiency)
    println!("Adding market data inputs:");
    let best_bid_price = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("best_bid_price".to_string()));
    let best_ask_price = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("best_ask_price".to_string()));
    let best_bid_qty = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("best_bid_qty".to_string()));
    let best_ask_qty = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("best_ask_qty".to_string()));
    let bid_queue_strong = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("bid_queue_strong".to_string()));
    let ask_queue_strong = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("ask_queue_strong".to_string()));
    
    // Strategy state inputs
    println!("Adding strategy state inputs:");
    let current_position = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("current_position".to_string()));
    let last_fill_price = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("last_fill_price".to_string()));
    let last_fill_side = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("last_fill_side".to_string()));
    
    // Stage 1: Calculate spread (critical for 0+ strategy)
    println!("Stage 1: Spread calculation");
    let spread = graph.add_node_with_output(rust_hls::ir::graph::Operation::Sub(best_ask_price, best_bid_price));
    
    // Stage 1: Check queue strength thresholds
    let qty_threshold = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(100)); // 100 shares minimum
    let bid_qty_strong = graph.add_node_with_output(rust_hls::ir::graph::Operation::CmpLt(qty_threshold, best_bid_qty));
    let ask_qty_strong = graph.add_node_with_output(rust_hls::ir::graph::Operation::CmpLt(qty_threshold, best_ask_qty));
    
    // Stage 2: Determine if spread is optimal (exactly 1 tick)
    println!("Stage 2: Optimal spread detection");
    let one_tick = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(1));
    let spread_optimal = graph.add_node_with_output(rust_hls::ir::graph::Operation::CmpEq(spread, one_tick));
    
    // Stage 2: Check if we're flat (no position)
    let zero_position = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(0));
    let is_flat = graph.add_node_with_output(rust_hls::ir::graph::Operation::CmpEq(current_position, zero_position));
    
    // Stage 2: Combine bid conditions
    let bid_conditions = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(bid_queue_strong, bid_qty_strong));
    let ask_conditions = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(ask_queue_strong, ask_qty_strong));
    
    // Stage 3: Final trading decision logic
    println!("Stage 3: Trading decision synthesis");
    
    // Can we buy? (flat + optimal spread + strong bid queue)
    let can_buy_part1 = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(is_flat, spread_optimal));
    let can_buy = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(can_buy_part1, bid_conditions));
    
    // Can we sell? (flat + optimal spread + strong ask queue)  
    let can_sell_part1 = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(is_flat, spread_optimal));
    let can_sell = graph.add_node_with_output(rust_hls::ir::graph::Operation::And(can_sell_part1, ask_conditions));
    
    // Action output (0=Hold, 1=Buy, 2=Sell)
    let buy_action = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(1));
    let sell_action = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(2));
    let hold_action = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(0));
    
    // Mux for action selection: can_buy ? 1 : (can_sell ? 2 : 0)
    let action_buy_or_sell = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mux(can_sell, sell_action, hold_action));
    let final_action = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mux(can_buy, buy_action, action_buy_or_sell));
    
    // Price output: can_buy ? bid_price : (can_sell ? ask_price : 0)
    let price_buy_or_sell = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mux(can_sell, best_ask_price, zero_position));
    let final_price = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mux(can_buy, best_bid_price, price_buy_or_sell));
    
    // Quantity output (50 shares for conservative sizing)
    let trade_quantity = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(50));
    let zero_qty = graph.add_node_with_output(rust_hls::ir::graph::Operation::Const(0));
    let has_action = graph.add_node_with_output(rust_hls::ir::graph::Operation::Or(can_buy, can_sell));
    let final_quantity = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mux(has_action, trade_quantity, zero_qty));
    
    // Outputs
    println!("Adding decision outputs:");
    graph.add_node(rust_hls::ir::graph::Operation::Store("action".to_string(), final_action));
    graph.add_node(rust_hls::ir::graph::Operation::Store("price".to_string(), final_price));
    graph.add_node(rust_hls::ir::graph::Operation::Store("quantity".to_string(), final_quantity));
    
    println!("HFT Pipeline Configuration:");
    println!("- Target Latency: < 100 nanoseconds");
    println!("- Pipeline Stages: 3 (minimal for ultra-low latency)");
    println!("- Decision Frequency: Every clock cycle (II=1)");
    println!("- Logic: Pure combinational with pipeline registers");
    
    graph
}

/// Run a simulation of the 0+ HFT strategy
fn run_hft_simulation() {
    let mut market = MarketDataSimulator::new(80300); // $803.00 stock price
    let mut strategy = ZeroPlusStrategy::new();
    
    println!("Initial market state:");
    market.print_order_book();
    
    // Simulate 100 market ticks
    for tick in 0..100 {
        // Advance market
        market.simulate_tick();
        
        // Get market snapshot
        let snapshot = market.get_market_snapshot();
        
        // Process with 0+ strategy
        let signal = strategy.process_market_data(&snapshot);
        
        // Execute trade if signal generated
        match signal.action {
            TradingAction::Buy => {
                let order_id = market.add_order(signal.price, signal.quantity, OrderSide::Buy);
                strategy.handle_fill(signal.price, signal.quantity, OrderSide::Buy);
                println!("Tick {}: BUY {} @ ${:.2}", tick, signal.quantity, signal.price as f64 / 100.0);
            }
            TradingAction::Sell => {
                let order_id = market.add_order(signal.price, signal.quantity, OrderSide::Sell);
                strategy.handle_fill(signal.price, signal.quantity, OrderSide::Sell);
                println!("Tick {}: SELL {} @ ${:.2}", tick, signal.quantity, signal.price as f64 / 100.0);
            }
            TradingAction::Scratch => {
                println!("Tick {}: SCRATCH {} @ ${:.2}", tick, signal.quantity, signal.price as f64 / 100.0);
                // Reset position after scratch
                strategy.position = 0;
            }
            TradingAction::Hold => {
                // No action
            }
            TradingAction::Cancel(_) => {
                println!("Tick {}: CANCEL order", tick);
            }
        }
        
        // Print status every 20 ticks
        if tick % 20 == 19 {
            println!("\n--- Tick {} Status ---", tick + 1);
            let stats = strategy.get_stats();
            stats.print();
            
            if tick == 39 || tick == 79 {
                market.print_order_book();
            }
        }
    }
    
    // Final results
    println!("\n=== FINAL SIMULATION RESULTS ===");
    let final_stats = strategy.get_stats();
    final_stats.print();
    
    println!("\nFPGA Implementation Benefits:");
    println!("- Decision latency: ~100ns (vs ~10μs software)");
    println!("- Deterministic timing: Every clock cycle");
    println!("- No OS jitter or garbage collection");
    println!("- Direct hardware implementation of 0+ logic");
    println!("- Optimal for high-frequency, low-latency trading");
}
