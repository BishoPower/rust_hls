// Testbench for HFT Zero Plus Strategy
`timescale 1ns / 1ps

module test_hft_zero_plus;
    // Parameters
    parameter DATA_WIDTH = 32;
    
    // Inputs
    reg ap_clk;
    reg ap_rst_n;
    reg ap_start;
    reg [DATA_WIDTH-1:0] best_bid_price;
    reg [DATA_WIDTH-1:0] best_ask_price;
    reg [DATA_WIDTH-1:0] best_bid_qty;
    reg [DATA_WIDTH-1:0] best_ask_qty;
    reg [DATA_WIDTH-1:0] bid_queue_strong;
    reg [DATA_WIDTH-1:0] ask_queue_strong;
    reg [DATA_WIDTH-1:0] current_position;
    reg [DATA_WIDTH-1:0] last_fill_price;
    reg [DATA_WIDTH-1:0] last_fill_side;
    
    // Outputs
    wire ap_done;
    wire ap_idle;
    wire ap_ready;
    wire [DATA_WIDTH-1:0] action;
    wire [DATA_WIDTH-1:0] price;
    wire [DATA_WIDTH-1:0] quantity;
    
    // Instantiate the Unit Under Test (UUT)
    hft_zero_plus #(
        .DATA_WIDTH(DATA_WIDTH)
    ) uut (
        .ap_clk(ap_clk),
        .ap_rst_n(ap_rst_n),
        .ap_start(ap_start),
        .best_bid_price(best_bid_price),
        .best_ask_price(best_ask_price),
        .best_bid_qty(best_bid_qty),
        .best_ask_qty(best_ask_qty),
        .bid_queue_strong(bid_queue_strong),
        .ask_queue_strong(ask_queue_strong),
        .current_position(current_position),
        .last_fill_price(last_fill_price),
        .last_fill_side(last_fill_side),
        .ap_done(ap_done),
        .ap_idle(ap_idle),
        .ap_ready(ap_ready),
        .action(action),
        .price(price),
        .quantity(quantity)
    );
    
    // Clock generation
    always begin
        ap_clk = 0;
        #5;
        ap_clk = 1;
        #5;
    end
    
    // Test stimulus
    initial begin
        // Initialize inputs
        ap_rst_n = 0;
        ap_start = 0;
        best_bid_price = 0;
        best_ask_price = 0;
        best_bid_qty = 0;
        best_ask_qty = 0;
        bid_queue_strong = 0;
        ask_queue_strong = 0;
        current_position = 0;
        last_fill_price = 0;
        last_fill_side = 0;
        
        // Wait for reset
        #100;
        ap_rst_n = 1;
        #20;
        
        $display("=== HFT Zero Plus Strategy Testbench ===");
        $display("Testing 0+ strategy decision logic...");
        
        // Test Case 1: Optimal conditions for BUY
        // - Spread = 1 tick (optimal)
        // - Position = 0 (flat)
        // - Strong bid queue
        $display("\nTest Case 1: Optimal BUY conditions");
        best_bid_price = 80299;  // $802.99
        best_ask_price = 80300;  // $803.00 (1 tick spread)
        best_bid_qty = 500;      // Strong queue (> 100)
        best_ask_qty = 200;      // Weaker ask queue
        bid_queue_strong = 1;    // Strong bid signal
        ask_queue_strong = 0;    // Weak ask signal
        current_position = 0;    // Flat position
        ap_start = 1;
        #10;
        ap_start = 0;
        #50;
        
        $display("Inputs: bid_price=%d, ask_price=%d, spread=%d", 
                 best_bid_price, best_ask_price, best_ask_price - best_bid_price);
        $display("Inputs: bid_qty=%d, ask_qty=%d, position=%d", 
                 best_bid_qty, best_ask_qty, current_position);
        $display("Outputs: action=%d (1=BUY), price=%d, quantity=%d", 
                 action, price, quantity);
        
        if (action == 1) begin
            $display("✓ PASS: Strategy decided to BUY");
            if (price == best_bid_price) $display("✓ PASS: Price set to bid price");
            else $display("✗ FAIL: Expected price=%d, got %d", best_bid_price, price);
            if (quantity == 50) $display("✓ PASS: Quantity set correctly");
            else $display("✗ FAIL: Expected quantity=50, got %d", quantity);
        end else begin
            $display("✗ FAIL: Expected BUY action (1), got %d", action);
        end
        
        // Test Case 2: Optimal conditions for SELL
        $display("\nTest Case 2: Optimal SELL conditions");
        best_bid_price = 80299;  // $802.99
        best_ask_price = 80300;  // $803.00 (1 tick spread)
        best_bid_qty = 200;      // Weaker bid queue
        best_ask_qty = 500;      // Strong ask queue (> 100)
        bid_queue_strong = 0;    // Weak bid signal
        ask_queue_strong = 1;    // Strong ask signal
        current_position = 0;    // Flat position
        ap_start = 1;
        #10;
        ap_start = 0;
        #50;
        
        $display("Inputs: bid_price=%d, ask_price=%d, spread=%d", 
                 best_bid_price, best_ask_price, best_ask_price - best_bid_price);
        $display("Outputs: action=%d (2=SELL), price=%d, quantity=%d", 
                 action, price, quantity);
        
        if (action == 2) begin
            $display("✓ PASS: Strategy decided to SELL");
            if (price == best_ask_price) $display("✓ PASS: Price set to ask price");
            else $display("✗ FAIL: Expected price=%d, got %d", best_ask_price, price);
        end else begin
            $display("✗ FAIL: Expected SELL action (2), got %d", action);
        end
        
        // Test Case 3: Wide spread (should HOLD)
        $display("\nTest Case 3: Wide spread (should hold)");
        best_bid_price = 80299;  // $802.99
        best_ask_price = 80301;  // $803.01 (2 tick spread - not optimal)
        best_bid_qty = 500;      // Strong queues
        best_ask_qty = 500;
        bid_queue_strong = 1;
        ask_queue_strong = 1;
        current_position = 0;    // Flat position
        ap_start = 1;
        #10;
        ap_start = 0;
        #50;
        
        $display("Inputs: spread=%d (not optimal)", best_ask_price - best_bid_price);
        $display("Outputs: action=%d (0=HOLD), price=%d, quantity=%d", 
                 action, price, quantity);
        
        if (action == 0) begin
            $display("✓ PASS: Strategy decided to HOLD (wide spread)");
            if (quantity == 0) $display("✓ PASS: Quantity set to 0");
        end else begin
            $display("✗ FAIL: Expected HOLD action (0), got %d", action);
        end
        
        // Test Case 4: Already have position (should HOLD)
        $display("\nTest Case 4: Non-flat position (should hold)");
        best_bid_price = 80299;  // $802.99
        best_ask_price = 80300;  // $803.00 (1 tick spread)
        best_bid_qty = 500;      // Strong queues
        best_ask_qty = 500;
        bid_queue_strong = 1;
        ask_queue_strong = 1;
        current_position = 50;   // Not flat - have position
        ap_start = 1;
        #10;
        ap_start = 0;
        #50;
        
        $display("Inputs: position=%d (not flat)", current_position);
        $display("Outputs: action=%d (0=HOLD), price=%d, quantity=%d", 
                 action, price, quantity);
        
        if (action == 0) begin
            $display("✓ PASS: Strategy decided to HOLD (have position)");
        end else begin
            $display("✗ FAIL: Expected HOLD action (0), got %d", action);
        end
        
        $display("\n=== Test Complete ===");
        $finish;
    end
    
    // Monitor changes
    initial begin
        $monitor("Time=%0t: action=%d, price=%d, quantity=%d", 
                 $time, action, price, quantity);
    end
    
endmodule
