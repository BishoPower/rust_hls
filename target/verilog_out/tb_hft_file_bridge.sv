/*
 * File-Based Testbench for HFT Zero Plus FPGA
 * ===========================================
 * SystemVerilog testbench that reads market data from files
 * Works with Python fpga_file_bridge.py for realistic simulation
 * No socket limitations - uses file I/O which works in Vivado
 * 
 * Usage in Vivado:
 * 1. Add this as simulation source
 * 2. Set as top module for simulation
 * 3. Run behavioral simulation
 * 4. Python writes market data to files
 */

`timescale 1ns / 1ps

module tb_hft_file_bridge;

    // Clock and reset
    logic ap_clk;
    logic ap_rst_n;
    
    // HFT FPGA Interface (matching actual hft_zero_plus.v)
    logic ap_start;
    logic ap_done;
    logic ap_idle;
    logic ap_ready;
    
    logic [31:0] best_bid_price;
    logic [31:0] best_ask_price;
    logic [31:0] best_bid_qty;
    logic [31:0] best_ask_qty;
    logic [31:0] bid_queue_strong;
    logic [31:0] ask_queue_strong;
    logic [31:0] current_position;
    logic [31:0] last_fill_price;
    logic [31:0] last_fill_side;
    
    // FPGA outputs (actual port names)
    logic [31:0] action;
    logic [31:0] price;
    logic [31:0] quantity;
    
    // File I/O
    integer market_data_file;
    integer fpga_output_file;
    string market_data_line;
    string fpga_output_line;
    
    // Simulation variables
    real bid_price_real, ask_price_real, target_price_real;
    integer bid_qty_int, ask_qty_int, trade_qty_int;
    integer market_data_count = 0;
    
    // Clock generation
    initial begin
        ap_clk = 0;
        forever #5 ap_clk = ~ap_clk; // 100MHz clock
    end
    
    // Instantiate HFT FPGA module (correct port mapping)
    hft_zero_plus #(
        .DATA_WIDTH(32)
    ) uut (
        .ap_clk(ap_clk),
        .ap_rst_n(ap_rst_n),
        .ap_start(ap_start),
        .ap_done(ap_done),
        .ap_idle(ap_idle),
        .ap_ready(ap_ready),
        .best_bid_price(best_bid_price),
        .best_ask_price(best_ask_price),
        .best_bid_qty(best_bid_qty),
        .best_ask_qty(best_ask_qty),
        .bid_queue_strong(bid_queue_strong),
        .ask_queue_strong(ask_queue_strong),
        .current_position(current_position),
        .last_fill_price(last_fill_price),
        .last_fill_side(last_fill_side),
        .action(action),
        .price(price),
        .quantity(quantity)
    );
    
    // Main simulation process
    initial begin
        $display("🚀 HFT FPGA File Bridge Testbench");
        $display("=================================");
        
        // Initialize signals
        ap_rst_n = 0;
        ap_start = 0;  // Don't start immediately
        best_bid_price = 0;
        best_ask_price = 0;
        best_bid_qty = 0;
        best_ask_qty = 0;
        bid_queue_strong = 0;
        ask_queue_strong = 0;
        current_position = 0;
        last_fill_price = 0;
        last_fill_side = 0;
        
        // Reset sequence
        #20 ap_rst_n = 1;
        #10; // Wait for reset to complete
        
        $display("📁 Starting file-based market data simulation...");
        $display("🎯 FPGA will be triggered when market data arrives");
        
        // Create bridge directory for file communication (simplified)
        // Note: Directory should already exist from Python script
        
        // Run simulation until we process some market data
        fork
            read_market_data();
            write_fpga_output();
            monitor_trading();
        join_none
        
        // Wait for some market data processing, then finish
        // Wait for multiple market updates to process all test cases
        wait(market_data_count >= 5);  // Process at least 5 market updates
        $display("💡 Processed %d market updates, waiting for FPGA to complete final...", market_data_count);
        
        // Wait much longer for FPGA to complete processing (ap_done to go high)
        // HLS modules can take many cycles depending on complexity
        for (int cycle_count = 0; cycle_count < 50; cycle_count++) begin
            @(posedge ap_clk);
            
            // Show progress every 10 cycles
            if ((cycle_count % 10) == 9) begin
                $display("⏳ Waiting... cycle %d/50, ap_done=%d, action=%d", 
                        cycle_count+1, ap_done, action);
            end
            
            // Exit early if FPGA completes
            if (ap_done) begin
                $display("✅ FPGA completed at cycle %d!", cycle_count+1);
                break;
            end
        end
        
        if (ap_done) begin
            $display("✅ FPGA completed processing!");
            $display("🎯 Final Results: action=%d, price=%d, quantity=%d", action, price, quantity);
        end else begin
            $display("⏰ FPGA still processing after 50 cycles");
            $display("🔧 Pipeline issue: Check ap_start timing and pipeline control");
        end
        
        #50_000_000;  // Additional 50ms for any final processing
        
        $display("✅ File bridge simulation complete!");
        $display("📊 Processed %d market data updates", market_data_count);
        $finish;
    end
    
    // Task to read market data from file
    task read_market_data();
        forever begin
            @(posedge ap_clk);
            
            // Only process when FPGA is ready
            if (ap_ready) begin
                // Try multiple possible file locations
                market_data_file = $fopen("fpga_bridge/market_data.json", "r");
                if (market_data_file == 0) begin
                    market_data_file = $fopen("../fpga_bridge/market_data.json", "r");
                end
                if (market_data_file == 0) begin
                    market_data_file = $fopen("../../fpga_bridge/market_data.json", "r");
                end
                
                if (market_data_file != 0) begin
                    // Parse JSON-like data (simplified for SystemVerilog)
                    while (!$feof(market_data_file)) begin
                        $fgets(market_data_line, market_data_file);
                        
                        // Look for price data in the line
                        if ($sscanf(market_data_line, "  \"bid_price\": %f,", bid_price_real) == 1) begin
                            best_bid_price = $rtoi(bid_price_real * 100); // Convert to fixed point
                        end
                        if ($sscanf(market_data_line, "  \"ask_price\": %f,", ask_price_real) == 1) begin
                            best_ask_price = $rtoi(ask_price_real * 100);
                        end
                        if ($sscanf(market_data_line, "  \"bid_qty\": %d,", bid_qty_int) == 1) begin
                            best_bid_qty = bid_qty_int;
                        end
                        if ($sscanf(market_data_line, "  \"ask_qty\": %d,", ask_qty_int) == 1) begin
                            best_ask_qty = ask_qty_int;
                            market_data_count++;
                            
                            // Set queue strength based on quantities (32-bit values)
                            bid_queue_strong = (bid_qty_int > 500) ? 32'h1 : 32'h0;
                            ask_queue_strong = (ask_qty_int > 500) ? 32'h1 : 32'h0;
                            
                            $display("📈 Market Update #%d: Bid $%0.2f/%d Ask $%0.2f/%d", 
                                    market_data_count, bid_price_real, bid_qty_int, 
                                    ask_price_real, ask_qty_int);
                            
                            // Show FPGA status before triggering
                            $display("🔍 FPGA Status: ap_ready=%b, ap_done=%b", ap_ready, ap_done);
                            
                            // Ensure inputs are stable before triggering
                            @(posedge ap_clk);
                            @(posedge ap_clk);  // Extra cycle for stability
                            
                            // HLS pipeline needs ap_start held until completion!
                            // Pipeline has 3 stages, needs 3+ cycles to flow through
                            ap_start = 1;
                            
                            $display("🚀 Triggered FPGA processing");
                            $display("🔍 Input Values: bid=$%d, ask=$%d, bid_qty=%d, ask_qty=%d", 
                                    best_bid_price, best_ask_price, best_bid_qty, best_ask_qty);
                            
                            // Wait for pipeline to complete (3 stages = 3+ cycles minimum)
                            repeat(5) @(posedge ap_clk);  // Wait 5 cycles for pipeline
                            ap_start = 0;  // Now we can release ap_start
                            
                            // Wait a bit and check FPGA outputs
                            #1000;  // Wait 1us
                            $display("🔍 FPGA Quick Check: action=%d, price=%d, quantity=%d", 
                                    action, price, quantity);
                        end
                    end
                    $fclose(market_data_file);
                end else begin
                    // Only show warning occasionally to avoid spam
                    if (market_data_count == 0) begin
                        $display("💡 Waiting for market data file: fpga_bridge/market_data.json");
                        $display("💡 Run: python ultimate_vivado_bridge.py");
                    end else if ((market_data_count % 10) == 0) begin
                        $display("💡 Waiting for more market data... (processed %d so far)", market_data_count);
                    end
                end
            end
            
            #5_000_000; // Check every 5ms (longer wait to allow Python to write new data)
        end
    endtask
    
    // Task to write FPGA output to file
    task write_fpga_output();
        forever begin
            @(posedge ap_done);  // Wait for FPGA to complete processing
            
            // Debug: Show all FPGA outputs
            $display("🔍 FPGA Debug: action=%d, price=%d, quantity=%d", action, price, quantity);
            
            if (action != 0) begin  // Check if FPGA has a decision
                fpga_output_file = $fopen("fpga_bridge/fpga_output.json", "w");
                if (fpga_output_file != 0) begin
                    target_price_real = $itor(price) / 100.0;
                    trade_qty_int = quantity;
                    
                    // Write JSON-like output
                    $fwrite(fpga_output_file, "{\n");
                    $fwrite(fpga_output_file, "  \"timestamp\": %0.3f,\n", $realtime / 1000000.0);
                    $fwrite(fpga_output_file, "  \"decision\": \"%s\",\n", 
                            (action == 1) ? "BUY" : (action == 2) ? "SELL" : "HOLD");
                    $fwrite(fpga_output_file, "  \"quantity\": %d,\n", trade_qty_int);
                    $fwrite(fpga_output_file, "  \"target_price\": %0.2f,\n", target_price_real);
                    $fwrite(fpga_output_file, "  \"latency_ns\": 50,\n");
                    $fwrite(fpga_output_file, "  \"market_timestamp\": %0.3f\n", $realtime / 1000000.0);
                    $fwrite(fpga_output_file, "}\n");
                    
                    $fclose(fpga_output_file);
                    
                    $display("⚡ FPGA Decision: %s %d @ $%0.2f", 
                            (action == 1) ? "BUY" : (action == 2) ? "SELL" : "HOLD", 
                            trade_qty_int, target_price_real);
                end
            end else begin
                $display("💭 FPGA completed processing but no trading action");
            end
        end
    endtask
    
    // Monitor trading activity
    task monitor_trading();
        integer trade_count = 0;
        integer prev_action = 0;
        integer prev_price = 0;
        integer prev_quantity = 0;
        
        forever begin
            @(posedge ap_clk);  // Check every clock cycle for changes
            
            // Monitor for any changes in FPGA outputs
            if (action != prev_action || price != prev_price || quantity != prev_quantity) begin
                $display("🔄 FPGA Output Change: action=%d, price=%d, quantity=%d", action, price, quantity);
                prev_action = action;
                prev_price = price;
                prev_quantity = quantity;
            end
            
            // Monitor completion
            if (ap_done && (action != 0)) begin
                trade_count++;
                $display("🎯 Trade #%d executed by FPGA (action=%d)", trade_count, action);
            end else if (ap_done) begin
                $display("💭 FPGA processing complete (no trade action)");
            end
        end
    endtask

endmodule
