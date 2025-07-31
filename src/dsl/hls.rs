//! High-level DSL for Rust HLS with pipeline support
//! 
//! This module provides a more user-friendly interface for creating
//! pipelined hardware descriptions in Rust.

use crate::ir::graph::{Graph, Operation};

/// HLS function builder with pipeline support
pub struct HLSFunction {
    pub graph: Graph,
    pub name: String,
}

impl HLSFunction {
    pub fn new(name: &str) -> Self {
        Self {
            graph: Graph::new(),
            name: name.to_string(),
        }
    }

    /// Enable pipelining with specified parameters
    pub fn pipeline(&mut self, ii: usize) -> &mut Self {
        self.graph.enable_pipeline(ii, 8, 1); // Default depth=8, unroll=1
        self
    }

    /// Enable pipelining with full control
    pub fn pipeline_advanced(&mut self, ii: usize, depth: usize, unroll: usize) -> &mut Self {
        self.graph.enable_pipeline(ii, depth, unroll);
        self
    }

    /// Add input port
    pub fn input(&mut self, name: &str) -> HLSValue {
        let value = self.graph.add_node_with_output(Operation::Load(name.to_string()));
        HLSValue { value, function: self }
    }

    /// Add output port
    pub fn output(&mut self, name: &str, value: HLSValue) {
        self.graph.add_node(Operation::Store(name.to_string(), value.value));
    }

    /// Generate Verilog with pipeline scheduling
    pub fn generate_verilog(&mut self) -> Result<String, String> {
        // Apply pipeline scheduling if enabled
        if self.graph.pipeline_config.enable {
            let mut scheduler = crate::passes::pipeline::PipelineScheduler::new();
            scheduler.schedule_pipeline(&mut self.graph)?;
        }

        Ok(crate::backend::verilog::generate_verilog_module(&self.graph, &self.name))
    }
}

/// Represents a value in the HLS function
pub struct HLSValue<'a> {
    pub value: crate::ir::graph::ValueId,
    function: &'a mut HLSFunction,
}

impl<'a> HLSValue<'a> {
    /// Add two values with automatic pipeline register insertion
    pub fn add(self, other: HLSValue) -> HLSValue<'a> {
        let result = self.function.graph.add_node_with_output(Operation::Add(self.value, other.value));
        HLSValue { value: result, function: self.function }
    }

    /// Multiply two values (uses DSP slices)
    pub fn mul(self, other: HLSValue) -> HLSValue<'a> {
        let result = self.function.graph.add_node_with_output(Operation::Mul(self.value, other.value));
        HLSValue { value: result, function: self.function }
    }

    /// Explicitly insert pipeline register
    pub fn pipeline_reg(self) -> HLSValue<'a> {
        let result = self.function.graph.insert_pipeline_register(self.value);
        HLSValue { value: result, function: self.function }
    }

    /// Create constant value
    pub fn constant(function: &'a mut HLSFunction, value: i64) -> HLSValue<'a> {
        let val = function.graph.add_node_with_output(Operation::Const(value));
        HLSValue { value: val, function }
    }
}

/// Macro for easy HLS function creation
#[macro_export]
macro_rules! hls_function {
    ($name:expr, |$func:ident| $body:expr) => {{
        let mut $func = $crate::dsl::hls::HLSFunction::new($name);
        $body;
        $func
    }};
}

/// Example usage macros
#[macro_export]
macro_rules! pipeline {
    ($ii:expr) => {
        |f: &mut HLSFunction| f.pipeline($ii)
    };
}
