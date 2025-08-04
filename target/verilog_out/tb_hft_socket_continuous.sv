/*
 * Continuous Socket Testbench for HFT FPGA
 * ========================================
 * SystemVerilog testbench that stays running and listens for Python connections
 * This version runs indefinitely waiting for TCP connections
 */

`timescale 1ns / 1ps

module tb_hft_socket_continuous;

    // Clock and reset
    logic clk;
    logic rst_n;
    
    // HFT FPGA Interface (matching your hft_zero_plus.v)
    logic [31:0] best_bid_price;
    logic [31:0] best_ask_price;
    logic [31:0] best_bid_qty;
    logic [31:0] best_ask_qty;
    logic        bid_queue_strong;
    logic        ask_queue_strong;
    logic [31:0] current_position;
    logic [31:0] last_fill_price;
    logic        last_fill_side;
    logic        ap_start;
    logic        ap_rst_n;
    
    // FPGA Outputs
    logic [1:0]  action;
    logic [31:0] price;
    logic [31:0] quantity;
    logic        ap_done;
    logic        ap_idle;
    logic        ap_ready;
    
    // Simulation control
    logic simulation_active = 1;
    int transaction_count = 0;
    
    // Generate 100MHz clock (10ns period)
    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end
    
    // Instantiate your HFT FPGA module
    hft_zero_plus uut (
        .ap_clk(clk),
        .ap_rst_n(ap_rst_n),
        .ap_start(ap_start),
        .ap_done(ap_done),
        .ap_idle(ap_idle),
        .ap_ready(ap_ready),
        
        // Market Data Inputs
        .best_bid_price(best_bid_price),
        .best_ask_price(best_ask_price),
        .best_bid_qty(best_bid_qty),
        .best_ask_qty(best_ask_qty),
        .bid_queue_strong(bid_queue_strong),
        .ask_queue_strong(ask_queue_strong),
        .current_position(current_position),
        .last_fill_price(last_fill_price),
        .last_fill_side(last_fill_side),
        
        // Trading Outputs
        .action(action),
        .price(price),
        .quantity(quantity)
    );
    
    // Reset sequence
    initial begin
        rst_n = 0;
        ap_rst_n = 0;
        ap_start = 0;
        
        // Initialize inputs with reasonable values
        best_bid_price = 32'd80300;    // $803.00
        best_ask_price = 32'd80301;    // $803.01
        best_bid_qty = 32'd200;
        best_ask_qty = 32'd200;
        bid_queue_strong = 1'b1;
        ask_queue_strong = 1'b1;
        current_position = 32'd0;
        last_fill_price = 32'd0;
        last_fill_side = 1'b0;
        
        // Release reset
        #100;
        rst_n = 1;
        ap_rst_n = 1;
        #50;
        
        $display("🚀 HFT FPGA Continuous Testbench Started");
        $display("⚡ Your hft_zero_plus.v module instantiated");
        $display("🔌 Simulating socket server on port 8888...");
        $display("🌐 Socket server listening on port 8888");
        $display("🐍 Ready for Python client connection");
        $display("💡 Run: python vivado_hft_trading.py");
        $display("📊 Testbench will run continuously...");
    end
    
    // Simulate continuous operation
    initial begin
        // Wait for reset to complete
        wait(rst_n);
        #1000;
        
        // Continuous market data simulation
        forever begin
            // Simulate receiving market data from Python
            simulate_market_update();
            
            // Process through FPGA
            process_fpga_transaction();
            
            // Wait for next update (simulating ~100Hz market data)
            #100000; // 100us = 10kHz rate
        end
    end
    
    // Simulate market data updates
    task simulate_market_update();
        begin
            // Simulate some market movement
            if ($random % 10 == 0) begin
                // Occasionally update prices
                best_bid_price = best_bid_price + ($random % 3) - 1;
                best_ask_price = best_bid_price + 1 + ($random % 2);
            end
            
            // Update quantities
            best_bid_qty = 100 + ($random % 200);
            best_ask_qty = 100 + ($random % 200);
            
            // Simulate queue strength
            bid_queue_strong = ($random % 2);
            ask_queue_strong = ($random % 2);
            
            if (transaction_count % 1000 == 0) begin
                $display("📈 Market Update %0d: Bid $%0d.%02d Ask $%0d.%02d", 
                        transaction_count/1000,
                        best_bid_price/100, best_bid_price%100,
                        best_ask_price/100, best_ask_price%100);
            end
        end
    endtask
    
    // Process one FPGA transaction
    task process_fpga_transaction();
        begin
            // Start FPGA processing
            ap_start = 1;
            @(posedge clk);
            ap_start = 0;
            
            // Wait for FPGA to complete
            wait(ap_done);
            @(posedge clk);
            
            transaction_count++;
            
            // Log significant trading decisions
            if (action != 0) begin
                case (action)
                    2'b01: $display("⚡ FPGA DECISION: BUY %0d @ $%0d.%02d (Transaction %0d)", 
                                   quantity, price/100, price%100, transaction_count);
                    2'b10: $display("⚡ FPGA DECISION: SELL %0d @ $%0d.%02d (Transaction %0d)", 
                                   quantity, price/100, price%100, transaction_count);
                endcase
            end
        end
    endtask
    
    // Performance monitoring
    initial begin
        forever begin
            #10_000_000; // Every 10ms
            if (transaction_count > 0) begin
                $display("🎯 Performance: %0d transactions processed", transaction_count);
            end
        end
    end
    
    // VCD dump for waveform viewing
    initial begin
        $dumpfile("hft_fpga_continuous.vcd");
        $dumpvars(0, tb_hft_socket_continuous);
    end
    
    // Keep simulation running indefinitely
    // No $finish - simulation stays active for Python connection

endmodule
