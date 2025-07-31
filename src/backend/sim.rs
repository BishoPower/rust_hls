//! RTL simulation stubs
//! 
//! This module provides basic simulation capabilities for generated RTL.

use crate::ir::graph::{Graph, Operation};
use std::collections::HashMap;

/// Simple simulation engine for IR graphs
pub struct Simulator {
    values: HashMap<usize, i64>, // ValueId -> actual value
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Set an input value
    pub fn set_input(&mut self, name: &str, value: i64, graph: &Graph) {
        // Find the input node and set its output value
        for node in &graph.nodes {
            if let Operation::Load(input_name) = &node.op {
                if input_name == name {
                    if let Some(output_id) = node.output {
                        self.values.insert(output_id.0, value);
                    }
                }
            }
        }
    }
    
    /// Run simulation on the graph
    pub fn simulate(&mut self, graph: &Graph) -> HashMap<String, i64> {
        let mut outputs = HashMap::new();
        
        // Process nodes in order (assuming they're already in dependency order)
        for node in &graph.nodes {
            match &node.op {
                Operation::Const(val) => {
                    if let Some(output_id) = node.output {
                        self.values.insert(output_id.0, *val);
                    }
                }
                Operation::Add(left, right) => {
                    if let Some(output_id) = node.output {
                        let left_val = self.values.get(&left.0).unwrap_or(&0);
                        let right_val = self.values.get(&right.0).unwrap_or(&0);
                        self.values.insert(output_id.0, left_val + right_val);
                    }
                }
                Operation::Sub(left, right) => {
                    if let Some(output_id) = node.output {
                        let left_val = self.values.get(&left.0).unwrap_or(&0);
                        let right_val = self.values.get(&right.0).unwrap_or(&0);
                        self.values.insert(output_id.0, left_val - right_val);
                    }
                }
                Operation::Mul(left, right) => {
                    if let Some(output_id) = node.output {
                        let left_val = self.values.get(&left.0).unwrap_or(&0);
                        let right_val = self.values.get(&right.0).unwrap_or(&0);
                        self.values.insert(output_id.0, left_val * right_val);
                    }
                }
                Operation::Store(name, value_id) => {
                    let value = self.values.get(&value_id.0).unwrap_or(&0);
                    outputs.insert(name.clone(), *value);
                }
                _ => {
                    // Handle other operations
                }
            }
        }
        
        outputs
    }
}
