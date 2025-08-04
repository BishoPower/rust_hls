// Generated for AMD Alveo U50 - SIMPLE VERSION
// synthesis translate_off
`timescale 1ns / 1ps
// synthesis translate_on

module simple_reference #(
    parameter integer DATA_WIDTH = 32,
    parameter integer ADDR_WIDTH = 16
) (
    // Clock and Reset
    input  wire                    ap_clk,
    input  wire                    ap_rst_n,
    
    // Control signals (HLS-style)
    input  wire                    ap_start,
    output reg                     ap_done,
    output wire                    ap_idle,
    output wire                    ap_ready,
    
    // Data inputs
    input  wire [DATA_WIDTH-1:0]  p,
    input  wire [DATA_WIDTH-1:0]  q,
    
    // Data outputs
    output reg  [DATA_WIDTH-1:0]  simple_add
);

    // Simple control state machine
    (* DONT_TOUCH = "yes" *) reg [1:0] state;
    localparam IDLE = 2'b00, COMPUTE = 2'b01, DONE = 2'b10;
    
    assign ap_idle = (state == IDLE);
    assign ap_ready = (state == IDLE);

endmodule
