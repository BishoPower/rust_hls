use rust_hls::ir::graph::Graph;
use rust_hls::passes::pipeline::PipelineScheduler;
use rust_hls::backend::verilog::generate_verilog_module;

fn main() {
    println!("Rust HLS Pipeline Demo");
    println!("======================");
    
    // Create a pipelined multiplier-accumulator (MAC) example
    let mut graph = create_pipelined_mac();
    
    // Enable pipelining with II=1, depth=4, unroll=1
    graph.enable_pipeline(1, 4, 1);
    
    // Schedule the pipeline
    let mut scheduler = PipelineScheduler::new();
    match scheduler.schedule_pipeline(&mut graph) {
        Ok(()) => {
            println!("Pipeline scheduling successful!");
            
            // Generate pipelined Verilog
            let verilog = generate_verilog_module(&graph, "pipelined_mac");
            
            // Write to file
            std::fs::create_dir_all("target/verilog_out").expect("Failed to create directory");
            std::fs::write("target/verilog_out/pipelined_mac.v", &verilog)
                .expect("Failed to write Verilog file");
                
            println!("Generated: target/verilog_out/pipelined_mac.v");
            println!("Ready for Vivado synthesis!");
            
            // Display file size
            if let Ok(metadata) = std::fs::metadata("target/verilog_out/pipelined_mac.v") {
                println!("File size: {} bytes", metadata.len());
            }
            
            // Show simulation instructions
            println!("\nNext Steps for Simulation:");
            println!("1. Open Vivado and create a new project");
            println!("2. Add target/verilog_out/pipelined_mac.v as source");
            println!("3. Create a testbench to verify MAC functionality");
            println!("4. Run behavioral simulation");
            println!("5. Synthesize for AMD Alveo U50 target");
        }
        Err(e) => {
            println!("Pipeline scheduling failed: {}", e);
        }
    }
}

/// Create a multiply-accumulate (MAC) computation graph
/// Implements: result = (a * b) + (c * d) + e
fn create_pipelined_mac() -> Graph {
    println!("\nCreating MAC Pipeline Graph");
    println!("Implementing: result = (a * b) + (c * d) + e");
    
    let mut graph = Graph::new();
    
    // Create input values using Load operations
    println!("Adding inputs: a, b, c, d, e (16-bit each)");
    let a = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("a".to_string()));      
    let b = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("b".to_string()));      
    let c = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("c".to_string()));      
    let d = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("d".to_string()));      
    let e = graph.add_node_with_output(rust_hls::ir::graph::Operation::Load("e".to_string()));      
    
    // First multiplication: a * b
    println!("Stage 1-2: First multiplication (a * b)");
    let mult1 = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mul(a, b));
    
    // Second multiplication: c * d  
    println!("Stage 1-2: Second multiplication (c * d)");
    let mult2 = graph.add_node_with_output(rust_hls::ir::graph::Operation::Mul(c, d));
    
    // First addition: (a * b) + (c * d)
    println!("Stage 3: First addition (a*b) + (c*d)");
    let add1 = graph.add_node_with_output(rust_hls::ir::graph::Operation::Add(mult1, mult2));
    
    // Final accumulation: ((a * b) + (c * d)) + e
    println!("Stage 4: Final accumulation + e");
    let result = graph.add_node_with_output(rust_hls::ir::graph::Operation::Add(add1, e));
    
    // Output port
    println!("Adding output: result (32-bit)");
    graph.add_node(rust_hls::ir::graph::Operation::Store("result".to_string(), result));
    
    println!("Operations: 2 multipliers, 2 adders");
    println!("Expected pipeline depth: 5 stages");
    println!("Target II: 1 (new input every cycle)");
    
    graph
}
