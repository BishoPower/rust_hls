/*
 * Real TCP Socket Testbench for Vivado
 * ===================================
 * SystemVerilog DPI-C based TCP socket server
 * Communicates with Python via real TCP sockets
 * 
 * Requirements:
 * - Vivado with DPI-C support
 * - C socket library compilation
 */

`timescale 1ns / 1ps

module tb_hft_socket_real;

    // Import DPI-C functions for socket communication
    import "DPI-C" function int socket_server_init(int port);
    import "DPI-C" function int socket_accept_client();
    import "DPI-C" function int socket_receive_data(output int data[]);
    import "DPI-C" function void socket_send_data(int data[]);
    import "DPI-C" function void socket_close();

    // Clock and reset
    logic clk;
    logic rst_n;
    
    // HFT FPGA Interface
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
    
    // Socket data arrays
    int input_data[8];   // Market data from Python
    int output_data[6];  // FPGA results to Python
    int socket_status;
    
    // Generate 100MHz clock
    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end
    
    // Instantiate HFT FPGA
    hft_zero_plus uut (
        .ap_clk(clk),
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
    
    // Initialize and run
    initial begin
        $display("🚀 Real TCP Socket HFT FPGA Testbench");
        
        // Reset
        rst_n = 0;
        ap_rst_n = 0;
        ap_start = 0;
        #100;
        rst_n = 1;
        ap_rst_n = 1;
        
        // Initialize socket server
        socket_status = socket_server_init(8888);
        if (socket_status < 0) begin
            $display("❌ Failed to create socket server");
            $finish;
        end
        
        $display("🌐 TCP Socket server listening on port 8888");
        $display("🐍 Ready for Python client: python vivado_hft_trading.py");
        
        // Accept client connection
        socket_status = socket_accept_client();
        if (socket_status < 0) begin
            $display("❌ Failed to accept client");
            $finish;
        end
        
        $display("✅ Python client connected");
        
        // Main processing loop
        forever begin
            // Receive market data from Python
            socket_status = socket_receive_data(input_data);
            
            if (socket_status > 0) begin
                // Parse received data
                best_bid_price = input_data[0];
                best_ask_price = input_data[1];
                best_bid_qty = input_data[2];
                best_ask_qty = input_data[3];
                bid_queue_strong = input_data[4];
                ask_queue_strong = input_data[5];
                current_position = input_data[6];
                
                // Process through FPGA
                ap_start = 1;
                @(posedge clk);
                ap_start = 0;
                
                // Wait for completion
                wait(ap_done);
                @(posedge clk);
                
                // Prepare output data
                output_data[0] = action;
                output_data[1] = price;
                output_data[2] = quantity;
                output_data[3] = ap_done;
                output_data[4] = ap_idle;
                output_data[5] = ap_ready;
                
                // Send back to Python
                socket_send_data(output_data);
                
                $display("📊 Processed: Bid $%0d Ask $%0d → Action %0d", 
                        best_bid_price, best_ask_price, action);
            end
            
            @(posedge clk);
        end
    end
    
    // Cleanup on finish
    final begin
        socket_close();
        $display("🔌 Socket closed");
    end

endmodule
