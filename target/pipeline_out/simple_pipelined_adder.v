// Generated for AMD Alveo U50 - PIPELINED VERSION
// Pipeline II: 1, Depth: 8, Stages: 4
// synthesis translate_off
`timescale 1ns / 1ps
// synthesis translate_on

module simple_pipelined_adder #(
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
    input  wire [DATA_WIDTH-1:0]  a,
    input  wire [DATA_WIDTH-1:0]  b,
    input  wire [DATA_WIDTH-1:0]  c,
    input  wire [DATA_WIDTH-1:0]  d,
    
    // Data outputs
    output reg  [DATA_WIDTH-1:0]  result
);

    // Pipeline control signals
    reg [4:0] pipeline_valid;
    reg [4:0] pipeline_counter;
    wire pipeline_ready;
    
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_0_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_0_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_0_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_0_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_1_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_1_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_1_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_1_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_2_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_2_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_2_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_2_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_3_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_3_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_3_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_3_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_4_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_4_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_4_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_4_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_5_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_5_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_5_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_5_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_6_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_6_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_6_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_6_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_7_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_7_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_7_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_7_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_8_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_8_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_8_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_8_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_9_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_9_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_9_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_9_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_10_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_10_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_10_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_10_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_11_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_11_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_11_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_11_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_12_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_12_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_12_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_12_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_13_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_13_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_13_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_13_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_14_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_14_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_14_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_14_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_15_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_15_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_15_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_15_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_16_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_16_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_16_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_16_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_17_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_17_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_17_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_17_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_18_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_18_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_18_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_18_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_19_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_19_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_19_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_19_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_20_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_20_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_20_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_20_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_21_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_21_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_21_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_21_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_22_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_22_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_22_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_22_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_23_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_23_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_23_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_23_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_24_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_24_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_24_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_24_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_25_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_25_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_25_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_25_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_26_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_26_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_26_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_26_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_27_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_27_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_27_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_27_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_28_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_28_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_28_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_28_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_29_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_29_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_29_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_29_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_30_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_30_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_30_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_30_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_31_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_31_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_31_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_31_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_32_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_32_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_32_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_32_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_33_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_33_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_33_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_33_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_34_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_34_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_34_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_34_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_35_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_35_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_35_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_35_stage_3;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_36_stage_0;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_36_stage_1;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_36_stage_2;
    (* KEEP = "true" *) reg [DATA_WIDTH-1:0] wire_36_stage_3;
    
    // Pipeline control
    assign ap_idle = (pipeline_counter == 0);
    assign ap_ready = pipeline_ready;
    assign pipeline_ready = (pipeline_counter < 1);
    
    // Pipeline control logic
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            pipeline_valid <= 0;
            pipeline_counter <= 0;
            ap_done <= 1'b0;
        end else begin
            // Shift pipeline valid bits
            pipeline_valid <= {pipeline_valid[3:0], ap_start};
            
            // Update counter
            if (ap_start) begin
                if (pipeline_counter < 8) begin
                    pipeline_counter <= pipeline_counter + 1;
                end
            end else if (pipeline_counter > 0 && pipeline_valid[4]) begin
                pipeline_counter <= pipeline_counter - 1;
            end
            
            // Output done signal
            ap_done <= pipeline_valid[4];
        end
    end
    
    // Pipeline Stage 0 (Cycle 0)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            wire_0_stage_0 <= {DATA_WIDTH{1'b0}};
            wire_3_stage_0 <= {DATA_WIDTH{1'b0}};
            wire_1_stage_0 <= {DATA_WIDTH{1'b0}};
            wire_4_stage_0 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[0]) begin
            wire_0_stage_0 <= a;
            wire_3_stage_0 <= c;
            wire_1_stage_0 <= b;
            wire_4_stage_0 <= d;
        end
    end
    
    // Pipeline Stage 1 (Cycle 2)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            wire_5_stage_1 <= {DATA_WIDTH{1'b0}};
            wire_2_stage_1 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[1]) begin
            wire_5_stage_1 <= wire_3_stage_0 + wire_4_stage_0;
            wire_2_stage_1 <= wire_0_stage_0 + wire_1_stage_0;
        end
    end
    
    // Pipeline Stage 2 (Cycle 3)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            wire_6_stage_2 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[2]) begin
            wire_6_stage_2 <= wire_2_stage_1 + wire_5_stage_1;
        end
    end
    
    // Pipeline Stage 3 (Cycle 4)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
        end else if (pipeline_valid[3]) begin
                    end
    end
    
    // Output assignments from final pipeline stage
    assign result = wire_6_stage_3;

endmodule
