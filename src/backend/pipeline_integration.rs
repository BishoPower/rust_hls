//! Complete pipeline integration and workflow
//! 
//! This module provides high-level functions to integrate pipeline scheduling
//! with Verilog generation for a complete HLS flow.

use crate::ir::graph::Graph;
use crate::passes::pipeline::run_pipeline_pass;
use crate::backend::verilog::generate_verilog_module;

/// Complete HLS flow: Schedule pipeline and generate Verilog
pub fn generate_pipelined_hls(mut graph: Graph, module_name: &str, ii: usize, depth: usize) -> Result<String, String> {
    // Enable pipelining configuration
    graph.enable_pipeline(ii, depth, 1);
    
    // Run pipeline scheduling pass
    run_pipeline_pass(&mut graph)?;
    
    // Generate Verilog with pipeline support
    let verilog = generate_verilog_module(&graph, module_name);
    
    Ok(verilog)
}

/// Generate simple (non-pipelined) HLS
pub fn generate_simple_hls(graph: Graph, module_name: &str) -> String {
    generate_verilog_module(&graph, module_name)
}

/// Pipeline configuration presets for common use cases
pub struct PipelinePresets;

impl PipelinePresets {
    /// High throughput: II=1, high depth for maximum throughput
    pub fn high_throughput() -> (usize, usize) {
        (1, 8)  // II=1, depth=8
    }
    
    /// Balanced: Moderate II and depth for balanced latency/throughput
    pub fn balanced() -> (usize, usize) {
        (2, 4)  // II=2, depth=4  
    }
    
    /// Low latency: Higher II, lower depth for reduced latency
    pub fn low_latency() -> (usize, usize) {
        (4, 2)  // II=4, depth=2
    }
    
    /// DSP intensive: Optimized for multiply-heavy workloads
    pub fn dsp_intensive() -> (usize, usize) {
        (1, 6)  // II=1, depth=6 (allows for DSP48 latency)
    }
}
