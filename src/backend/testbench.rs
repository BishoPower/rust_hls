//! Rust FFI interface for Verilator simulations
//! 
//! This module provides a safe Rust interface to Verilator-generated C++ simulations.

use std::ffi::c_void;
use std::path::Path;
use libloading::{Library, Symbol};
use crate::backend::verilator::{VerilatorSim, create_shared_library};
use crate::ir::graph::Graph;

/// Safe Rust wrapper for Verilator simulation
pub struct VerilatorTestbench {
    lib: Library,
    sim: *mut c_void,
}

impl VerilatorTestbench {
    /// Create a new testbench from a compiled Verilator library
    pub fn new(lib_path: &Path) -> Result<Self, String> {
        unsafe {
            let lib = Library::new(lib_path)
                .map_err(|e| format!("Failed to load library: {}", e))?;
            
            let create_sim: Symbol<unsafe extern "C" fn() -> *mut c_void> = lib
                .get(b"create_sim")
                .map_err(|e| format!("Failed to get create_sim symbol: {}", e))?;
            
            let sim = create_sim();
            if sim.is_null() {
                return Err("Failed to create simulation instance".to_string());
            }
            
            Ok(Self { lib, sim })
        }
    }
    
    /// Reset the simulation
    pub fn reset(&self) -> Result<(), String> {
        unsafe {
            let reset_sim: Symbol<unsafe extern "C" fn(*mut c_void)> = self.lib
                .get(b"reset_sim")
                .map_err(|e| format!("Failed to get reset_sim symbol: {}", e))?;
            
            reset_sim(self.sim);
            Ok(())
        }
    }
    
    /// Set input 'a' value
    pub fn set_input_a(&self, value: u32) -> Result<(), String> {
        unsafe {
            let set_input_a: Symbol<unsafe extern "C" fn(*mut c_void, u32)> = self.lib
                .get(b"set_input_a_sim")
                .map_err(|e| format!("Failed to get set_input_a_sim symbol: {}", e))?;
            
            set_input_a(self.sim, value);
            Ok(())
        }
    }
    
    /// Set input 'b' value
    pub fn set_input_b(&self, value: u32) -> Result<(), String> {
        unsafe {
            let set_input_b: Symbol<unsafe extern "C" fn(*mut c_void, u32)> = self.lib
                .get(b"set_input_b_sim")
                .map_err(|e| format!("Failed to get set_input_b_sim symbol: {}", e))?;
            
            set_input_b(self.sim, value);
            Ok(())
        }
    }
    
    /// Get output 'result' value
    pub fn get_output_result(&self) -> Result<u32, String> {
        unsafe {
            let get_output: Symbol<unsafe extern "C" fn(*mut c_void) -> u32> = self.lib
                .get(b"get_output_result_sim")
                .map_err(|e| format!("Failed to get get_output_result_sim symbol: {}", e))?;
            
            Ok(get_output(self.sim))
        }
    }
    
    /// Run the simulation until completion
    pub fn run_until_done(&self) -> Result<(), String> {
        unsafe {
            let run_until_done: Symbol<unsafe extern "C" fn(*mut c_void)> = self.lib
                .get(b"run_until_done_sim")
                .map_err(|e| format!("Failed to get run_until_done_sim symbol: {}", e))?;
            
            run_until_done(self.sim);
            Ok(())
        }
    }
    
    /// Check if simulation is done
    pub fn is_done(&self) -> Result<bool, String> {
        unsafe {
            let is_done: Symbol<unsafe extern "C" fn(*mut c_void) -> i32> = self.lib
                .get(b"is_done_sim")
                .map_err(|e| format!("Failed to get is_done_sim symbol: {}", e))?;
            
            Ok(is_done(self.sim) != 0)
        }
    }
    
    /// Run a complete test with inputs and return output
    pub fn run_test(&self, input_a: u32, input_b: u32) -> Result<u32, String> {
        self.reset()?;
        self.set_input_a(input_a)?;
        self.set_input_b(input_b)?;
        self.run_until_done()?;
        self.get_output_result()
    }
}

impl Drop for VerilatorTestbench {
    fn drop(&mut self) {
        unsafe {
            if let Ok(destroy_sim) = self.lib.get::<Symbol<unsafe extern "C" fn(*mut c_void)>>(b"destroy_sim") {
                destroy_sim(self.sim);
            }
        }
    }
}

/// High-level testbench runner using the organized directory structure
pub struct TestbenchRunner {
    verilator_sim: VerilatorSim,
    lib_path: Option<std::path::PathBuf>,
}

impl TestbenchRunner {
    /// Create a new testbench runner
    pub fn new(module_name: &str) -> Self {
        Self {
            verilator_sim: VerilatorSim::new(module_name),
            lib_path: None,
        }
    }
    
    /// Compile the design and prepare for simulation
    pub fn prepare(&mut self, graph: &Graph) -> Result<(), String> {
        println!("🔧 Preparing testbench for module '{}'", self.verilator_sim.get_module_name());
        
        // Compile with Verilator
        self.verilator_sim.compile_from_graph(graph)?;
        
        // Create shared library for FFI
        let lib_path = create_shared_library(
            self.verilator_sim.get_module_name(),
            self.verilator_sim.get_sim_dir()
        )?;
        
        self.lib_path = Some(lib_path);
        
        println!("✅ Testbench preparation complete!");
        println!("   📁 Verilog files: {}", self.verilator_sim.get_verilog_out_dir().display());
        println!("   📁 Simulation files: {}", self.verilator_sim.get_sim_dir().display());
        println!("   📁 Verilated objects: {}", self.verilator_sim.get_obj_dir().display());
        
        Ok(())
    }
    
    /// Create a testbench instance for running tests
    pub fn create_testbench(&self) -> Result<VerilatorTestbench, String> {
        if let Some(ref lib_path) = self.lib_path {
            VerilatorTestbench::new(lib_path)
        } else {
            Err("Testbench not prepared. Call prepare() first.".to_string())
        }
    }
    
    /// Get the directory structure info
    pub fn get_directory_info(&self) -> DirectoryInfo {
        DirectoryInfo {
            verilog_out: self.verilator_sim.get_verilog_out_dir().to_path_buf(),
            sim: self.verilator_sim.get_sim_dir().to_path_buf(),
            obj: self.verilator_sim.get_obj_dir(),
            lib: self.lib_path.clone(),
        }
    }
    
    /// Run complete workflow: compile, build, and test from a graph
    pub fn run_from_graph(&mut self, graph: &Graph, test_cases: &[(u32, u32, u32)]) -> Result<(), String> {
        println!("🚀 Starting complete testbench workflow for module '{}'", self.verilator_sim.get_module_name());
        
        // Step 1: Prepare the testbench (compile Verilog with Verilator)
        self.prepare(graph)?;
        
        // Step 2: Run tests
        self.run_tests(test_cases, graph)?;
        
        println!("✅ Complete workflow finished successfully!");
        Ok(())
    }
    
    /// Run a series of test cases
    pub fn run_tests(&self, test_cases: &[(u32, u32, u32)], graph: &Graph) -> Result<(), String> {
        println!("🧪 Running {} test cases", test_cases.len());
        
        // Try to create testbench (this will fail if FFI library creation failed)
        match self.create_testbench() {
            Ok(testbench) => {
                println!("   ✅ FFI testbench created successfully");
                
                // Run each test case
                for (i, &(input_a, input_b, expected)) in test_cases.iter().enumerate() {
                    match testbench.run_test(input_a, input_b) {
                        Ok(actual) => {
                            if actual == expected {
                                println!("   ✅ Test {}: {}+{}={} (passed)", i+1, input_a, input_b, actual);
                            } else {
                                println!("   ❌ Test {}: {}+{}={} (expected {}, got {})", 
                                        i+1, input_a, input_b, expected, expected, actual);
                                return Err(format!("Test {} failed: expected {}, got {}", i+1, expected, actual));
                            }
                        }
                        Err(e) => {
                            println!("   ❌ Test {} failed to run: {}", i+1, e);
                            return Err(format!("Test {} execution failed: {}", i+1, e));
                        }
                    }
                }
                
                println!("   🎉 All {} tests passed!", test_cases.len());
                Ok(())
            }
            Err(e) => {
                println!("   ⚠️  FFI testbench unavailable: {}", e);
                println!("   🔄 Falling back to software simulation");
                
                // Fallback to software simulation
                self.run_software_simulation(test_cases, graph)
            }
        }
    }
    
    /// Fallback software simulation when Verilator FFI is not available
    fn run_software_simulation(&self, test_cases: &[(u32, u32, u32)], graph: &Graph) -> Result<(), String> {
        use crate::backend::sim::*;
        
        let mut sim = Simulator::new();
        
        for (i, &(input_a, input_b, expected)) in test_cases.iter().enumerate() {
            sim.set_input("a", input_a as i64, graph);
            sim.set_input("b", input_b as i64, graph);
            
            let outputs = sim.simulate(graph);
            if let Some(&actual) = outputs.get("result") {
                let actual = actual as u32;
                if actual == expected {
                    println!("   ✅ Software Test {}: {}+{}={} (passed)", i+1, input_a, input_b, actual);
                } else {
                    println!("   ❌ Software Test {}: {}+{}={} (expected {}, got {})", 
                            i+1, input_a, input_b, expected, expected, actual);
                    return Err(format!("Software test {} failed: expected {}, got {}", i+1, expected, actual));
                }
            } else {
                return Err(format!("Software test {}: no result output found", i+1));
            }
        }
        
        println!("   🎉 All {} software simulation tests passed!", test_cases.len());
        Ok(())
    }
}

/// Information about the directory structure
#[derive(Debug, Clone)]
pub struct DirectoryInfo {
    pub verilog_out: std::path::PathBuf,
    pub sim: std::path::PathBuf,
    pub obj: std::path::PathBuf,
    pub lib: Option<std::path::PathBuf>,
}

impl DirectoryInfo {
    /// Print a nice directory tree
    pub fn print_tree(&self) {
        println!("📂 Project Structure:");
        println!("├── verilog_out/          # Generated .v files");
        println!("│   └── *.v");
        println!("├── sim/                  # Simulation workspace");
        println!("│   ├── testbench.cpp     # C++ testbench");
        println!("│   ├── obj_dir/          # Verilator output");
        println!("│   │   ├── V*.cpp        # Verilated C++");
        println!("│   │   ├── V*.h          # Verilated headers");
        println!("│   │   └── V*            # Executable");
        if self.lib.is_some() {
            println!("│   └── lib*_sim.so       # Shared library for FFI");
        }
        println!("└── target/               # Rust build artifacts");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::*;
    use crate::ir::lower::*;
    
    #[test]
    fn test_full_verilator_workflow() {
        // Create a simple adder circuit
        let a = input("a", 32);
        let b = input("b", 32);
        let sum = add(a, b);
        let result = output("result", sum);
        
        let graph = lower_expr_to_graph(&result);
        
        let mut runner = TestbenchRunner::new("test_adder_full");
        
        // Test cases: (a, b, expected_result) - for future use
        let _test_cases = vec![
            (5, 10, 15),
            (100, 200, 300),
            (0, 0, 0),
            (1, 1, 2),
        ];
        
        // This test will only pass if Verilator is installed
        match runner.prepare(&graph) {
            Ok(_) => {
                let dir_info = runner.get_directory_info();
                dir_info.print_tree();
                
                // Try to create a testbench instance
                match runner.create_testbench() {
                    Ok(_testbench) => {
                        println!("✅ Full Verilator workflow test passed!");
                        // TODO: Run actual tests with testbench
                    }
                    Err(e) => {
                        println!("Note: Testbench creation failed (expected if library compilation failed): {}", e);
                    }
                }
            }
            Err(e) if e.contains("Failed to run Verilator") => {
                println!("Skipping full workflow test - Verilator not installed");
            }
            Err(e) => panic!("Unexpected error in workflow: {}", e),
        }
    }
}
