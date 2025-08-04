`timescale 1ns / 1ps

module tb_hft_continuous_bridge;

    // HLS Interface Signals
    reg         ap_clk;
    reg         ap_rst_n;
    reg         ap_start;
    wire        ap_done;
    wire        ap_idle;
    wire        ap_ready;
    
    // Market Data Inputs to HFT Module (matching actual ports)
    reg [31:0]  best_bid_price;
    reg [31:0]  best_ask_price;
    reg [31:0]  best_bid_qty;
    reg [31:0]  best_ask_qty;
    reg [31:0]  bid_queue_strong;
    reg [31:0]  ask_queue_strong;
    reg [31:0]  current_position;
    reg [31:0]  last_fill_price;
    reg [31:0]  last_fill_side;
    
    // Trading Decision Outputs (matching actual ports)
    wire [31:0] action;
    wire [31:0] price;
    wire [31:0] quantity;
    
    // File handling variables
    integer file_handle;
    integer scan_result;
    string line_buffer;
    real timestamp_current, timestamp_last;
    integer market_update_count;
    time last_file_time, current_file_time;
    
    // JSON parsing variables
    string json_content;
    real parsed_timestamp;
    real parsed_bid_price, parsed_ask_price;
    integer parsed_bid_qty, parsed_ask_qty;
    real parsed_spread;
    
    // HFT Zero Plus Module Instance
    hft_zero_plus uut (
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
    
    // Clock Generation
    always #5 ap_clk = ~ap_clk;
    
    // JSON parsing function (simplified)
    function automatic integer parse_market_data();
        string temp_line;
        integer char_pos;
        
        file_handle = $fopen("fpga_bridge/market_data.json", "r");
        if (file_handle == 0) begin
            $display("❌ ERROR: Cannot open market_data.json");
            return 0;
        end
        
        // Initialize values
        parsed_timestamp = 0.0;
        parsed_bid_price = 0.0;
        parsed_ask_price = 0.0;
        parsed_bid_qty = 0;
        parsed_ask_qty = 0;
        parsed_spread = 0.0;
        
        // Simple line-by-line parsing
        while (!$feof(file_handle)) begin
            scan_result = $fgets(temp_line, file_handle);
            if (scan_result > 0) begin
                // Parse timestamp
                if ($sscanf(temp_line, "  \"timestamp\": %f,", parsed_timestamp) == 1) begin
                    // Found timestamp
                end
                // Parse bid_price  
                else if ($sscanf(temp_line, "  \"bid_price\": %f,", parsed_bid_price) == 1) begin
                    // Found bid price
                end
                // Parse ask_price
                else if ($sscanf(temp_line, "  \"ask_price\": %f,", parsed_ask_price) == 1) begin
                    // Found ask price
                end
                // Parse bid_qty
                else if ($sscanf(temp_line, "  \"bid_qty\": %d,", parsed_bid_qty) == 1) begin
                    // Found bid quantity
                end
                // Parse ask_qty
                else if ($sscanf(temp_line, "  \"ask_qty\": %d,", parsed_ask_qty) == 1) begin
                    // Found ask quantity
                end
                // Parse spread
                else if ($sscanf(temp_line, "  \"spread\": %f", parsed_spread) == 1) begin
                    // Found spread (last item, no comma)
                end
            end
        end
        
        $fclose(file_handle);
        
        // Validate we got data
        if (parsed_timestamp > 0 && parsed_bid_price > 0 && parsed_ask_price > 0) begin
            return 1;
        end else begin
            $display("❌ ERROR: Failed to parse valid market data");
            return 0;
        end
    endfunction
    
    // Process HLS handshake
    task automatic process_hls_transaction();
        $display("🔄 Starting HLS transaction...");
        
        // Convert to fixed-point (multiply by 100 for 2 decimal places)
        best_bid_price = $rtoi(parsed_bid_price * 100);
        best_ask_price = $rtoi(parsed_ask_price * 100);
        best_bid_qty = parsed_bid_qty;
        best_ask_qty = parsed_ask_qty;
        bid_queue_strong = (parsed_bid_qty > 100) ? 32'd1 : 32'd0;  // Strong if >100
        ask_queue_strong = (parsed_ask_qty > 100) ? 32'd1 : 32'd0;  // Strong if >100
        current_position = 32'd0;  // Start with no position
        last_fill_price = 32'd0;   // No previous fills
        last_fill_side = 32'd0;    // No previous side
        
        $display("📊 Market Data Input:");
        $display("   💹 Bid: $%.2f (%0d)", parsed_bid_price, best_bid_price);
        $display("   💹 Ask: $%.2f (%0d)", parsed_ask_price, best_ask_price);
        $display("   📊 Bid Qty: %0d (Strong: %0d)", best_bid_qty, bid_queue_strong);
        $display("   📊 Ask Qty: %0d (Strong: %0d)", best_ask_qty, ask_queue_strong);
        $display("   📏 Spread: $%.3f", parsed_spread);
        
        // Wait for HLS ready
        wait(ap_ready == 1'b1);
        $display("✅ HLS Ready - Starting transaction");
        
        // Start HLS processing
        ap_start = 1'b1;
        @(posedge ap_clk);
        ap_start = 1'b0;
        
        // Wait for completion
        wait(ap_done == 1'b1);
        $display("✅ HLS Done - Transaction complete");
        
        // Display results
        $display("🎯 Trading Decision:");
        $display("   🎬 Action: %0d (0=None, 1=Buy, 2=Sell)", action);
        $display("   💰 Price: $%.2f", price / 100.0);
        $display("   � Quantity: %0d", quantity);
        
        // Wait for idle
        wait(ap_idle == 1'b1);
        $display("⏸️  HLS Idle - Ready for next transaction\n");
    endtask
    
    // Main test sequence
    initial begin
        $display("🚀 CONTINUOUS HFT FILE BRIDGE TESTBENCH");
        $display("=========================================");
        
        // Initialize signals
        ap_clk = 0;
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
        timestamp_last = 0.0;
        market_update_count = 0;
        last_file_time = 0;
        
        // Reset sequence
        $display("🔄 Applying reset...");
        repeat(10) @(posedge ap_clk);
        ap_rst_n = 1;
        repeat(5) @(posedge ap_clk);
        $display("✅ Reset complete");
        
        // Wait for initial file
        $display("📁 Waiting for initial market data file...");
        while (1) begin
            if (parse_market_data()) begin
                timestamp_last = parsed_timestamp;
                market_update_count++;
                $display("📈 Market Update #%0d (timestamp: %.3f)", market_update_count, parsed_timestamp);
                process_hls_transaction();
                break;
            end
            #1000000; // Wait 1ms before checking again
        end
        
        // Continuous monitoring loop
        $display("🔍 Starting continuous file monitoring...");
        while (1) begin
            // Check for file updates every 100ms
            #100000000; // 100ms
            
            if (parse_market_data()) begin
                // Check if timestamp changed (new data)
                if (parsed_timestamp != timestamp_last) begin
                    timestamp_last = parsed_timestamp;
                    market_update_count++;
                    $display("📈 Market Update #%0d (timestamp: %.3f)", market_update_count, parsed_timestamp);
                    process_hls_transaction();
                    
                    // If we've processed many updates, stop for demo
                    if (market_update_count >= 10) begin
                        $display("🎯 Processed %0d market updates - Demo complete", market_update_count);
                        break;
                    end
                end
            end
        end
        
        $display("✅ Continuous bridge test completed!");
        $finish;
    end
    
    // Timeout protection
    initial begin
        #5000000000; // 5 seconds max (within 32-bit range)
        $display("⏰ TIMEOUT: Test completed after 5 seconds");
        $finish;
    end

endmodule
