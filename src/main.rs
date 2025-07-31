// Main entry point - see examples/ directory for comprehensive demos
// Run: cargo run --example pipelined_mac

fn main() {
    println!("Rust HLS Compiler");
    println!("=================");
    println!("This is a High-Level Synthesis compiler for FPGA development.");
    println!("");
    println!("Available examples:");
    println!("  cargo run --example pipelined_mac        # Complete MAC pipeline");
    println!("");
    println!("Generated Verilog will be in target/verilog_out/");
    println!("Optimized for AMD Alveo U50 and Vivado 2025");
}
