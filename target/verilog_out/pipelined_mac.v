// Generated for AMD Alveo U50 - PIPELINED VERSION (CLEAN)
// Pipeline: 5-stage MAC implementation
// synthesis translate_off
`timescale 1ns / 1ps
// synthesis translate_on

module pipelined_mac #(
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
    
    // Data inputs - MAC: result = (a * b) + (c * d) + e
    input  wire [DATA_WIDTH-1:0]  a,
    input  wire [DATA_WIDTH-1:0]  b,
    input  wire [DATA_WIDTH-1:0]  c,
    input  wire [DATA_WIDTH-1:0]  d,
    input  wire [DATA_WIDTH-1:0]  e,
    
    // Data outputs
    output wire [DATA_WIDTH-1:0]  result
);

    // Pipeline control signals
    reg [4:0] pipeline_valid;  // 5-stage pipeline
    reg [3:0] pipeline_counter;
    
    // Pipeline registers for Stage 0 (Input Registration)
    reg [DATA_WIDTH-1:0] a_reg0;
    reg [DATA_WIDTH-1:0] b_reg0;
    reg [DATA_WIDTH-1:0] c_reg0;
    reg [DATA_WIDTH-1:0] d_reg0;
    reg [DATA_WIDTH-1:0] e_reg0;
    
    // Pipeline registers for Stage 1 (Multiplication)
    reg [DATA_WIDTH-1:0] mult_ab_reg1, mult_cd_reg1;
    reg [DATA_WIDTH-1:0] e_reg1;
    
    // Pipeline registers for Stage 2 (First Addition)
    reg [DATA_WIDTH-1:0] add_mult_reg2;
    reg [DATA_WIDTH-1:0] e_reg2;
    
    // Pipeline registers for Stage 3 (Final Addition)
    reg [DATA_WIDTH-1:0] result_reg3;
    
    // Control logic
    assign ap_idle = (pipeline_counter == 0);
    assign ap_ready = (pipeline_counter < 5);  // Can accept new input when not full
    
    // Pipeline control logic
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            pipeline_valid <= 5'b00000;
            pipeline_counter <= 4'b0000;
            ap_done <= 1'b0;
        end else begin
            // Shift pipeline valid bits
            pipeline_valid <= {pipeline_valid[3:0], ap_start && ap_ready};
            
            // Update counter
            if (ap_start && ap_ready) begin
                if (pipeline_counter < 5) begin
                    pipeline_counter <= pipeline_counter + 1;
                end
            end else if (pipeline_counter > 0 && pipeline_valid[4]) begin
                pipeline_counter <= pipeline_counter - 1;
            end
            
            // Output done signal when result emerges from pipeline
            ap_done <= pipeline_valid[4];
        end
    end
    
    // Pipeline Stage 0: Input Registration
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            a_reg0 <= {DATA_WIDTH{1'b0}};
            b_reg0 <= {DATA_WIDTH{1'b0}};
            c_reg0 <= {DATA_WIDTH{1'b0}};
            d_reg0 <= {DATA_WIDTH{1'b0}};
            e_reg0 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[0]) begin
            a_reg0 <= a;
            b_reg0 <= b;
            c_reg0 <= c;
            d_reg0 <= d;
            e_reg0 <= e;
        end
    end
    
    // Pipeline Stage 1: Parallel Multiplications (DSP48E2 optimized for AU50)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            mult_ab_reg1 <= {DATA_WIDTH{1'b0}};
            mult_cd_reg1 <= {DATA_WIDTH{1'b0}};
            e_reg1 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[1]) begin
            // Force DSP48E2 usage for AU50 optimization
            (* USE_DSP = "yes", DSP_A_INPUT = "DIRECT", DSP_B_INPUT = "DIRECT" *) 
            mult_ab_reg1 <= a_reg0 * b_reg0;
            (* USE_DSP = "yes", DSP_A_INPUT = "DIRECT", DSP_B_INPUT = "DIRECT" *) 
            mult_cd_reg1 <= c_reg0 * d_reg0;
            e_reg1 <= e_reg0;  // Pass through
        end
    end
    
    // Pipeline Stage 2: First Addition (mult_ab + mult_cd)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            add_mult_reg2 <= {DATA_WIDTH{1'b0}};
            e_reg2 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[2]) begin
            add_mult_reg2 <= mult_ab_reg1 + mult_cd_reg1;
            e_reg2 <= e_reg1;  // Pass through
        end
    end
    
    // Pipeline Stage 3: Final Addition (result = (a*b + c*d) + e)
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            result_reg3 <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[3]) begin
            result_reg3 <= add_mult_reg2 + e_reg2;
        end
    end
    
    // Pipeline Stage 4: Output Assignment
    always @(posedge ap_clk) begin
        if (!ap_rst_n) begin
            result <= {DATA_WIDTH{1'b0}};
        end else if (pipeline_valid[4]) begin
            result <= result_reg3;
        end
    end

endmodule
