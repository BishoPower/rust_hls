//! Verilator integration module
//! 
//! This module provides integration with Verilator to compile generated Verilog
//! into C++ simulation models and run testbenches from Rust.

use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use crate::backend::verilog::generate_verilog_module;
use crate::ir::graph::Graph;

/// Verilator simulation wrapper
pub struct VerilatorSim {
    module_name: String,
    verilog_out_dir: PathBuf,
    sim_dir: PathBuf,
    verilated_executable: Option<PathBuf>,
}

impl VerilatorSim {
    /// Create a new Verilator simulation with organized directory structure
    pub fn new(module_name: &str) -> Self {
        let base_dir = PathBuf::from("target");
        let verilog_out_dir = base_dir.join("verilog_out");
        let sim_dir = base_dir.join("sim").join(module_name);
        
        Self {
            module_name: module_name.to_string(),
            verilog_out_dir,
            sim_dir,
            verilated_executable: None,
        }
    }
    
    /// Generate Verilog and compile with Verilator
    pub fn compile_from_graph(&mut self, graph: &Graph) -> Result<(), String> {
        // Create directories
        fs::create_dir_all(&self.verilog_out_dir)
            .map_err(|e| format!("Failed to create verilog_out directory: {}", e))?;
        fs::create_dir_all(&self.sim_dir)
            .map_err(|e| format!("Failed to create sim directory: {}", e))?;
        
        // Generate Verilog to verilog_out/
        let verilog_code = generate_verilog_module(graph, &self.module_name);
        let verilog_path = self.verilog_out_dir.join(format!("{}.v", self.module_name));
        
        fs::write(&verilog_path, verilog_code)
            .map_err(|e| format!("Failed to write Verilog file: {}", e))?;
        
        println!("Generated Verilog: {}", verilog_path.display());
        
        // Generate C++ testbench to sim/
        self.generate_cpp_testbench()?;
        
        // Run Verilator (output goes to sim/)
        self.run_verilator(&verilog_path)?;
        
        // Compile the generated C++
        self.compile_cpp()?;
        
        Ok(())
    }
    
    /// Generate C++ testbench for the Verilated module
    fn generate_cpp_testbench(&self) -> Result<(), String> {
        let cpp_code = format!(r#"
// Generated C++ testbench wrapper for {}
#include "V{}.h"
#include "verilated.h"
#include "verilated_vcd_c.h"
#include <iostream>
#include <memory>

class {}Sim {{
private:
    std::unique_ptr<V{}> dut;
    std::unique_ptr<VerilatedVcdC> trace;
    uint64_t sim_time;
    
public:
    {}Sim() : sim_time(0) {{
        dut = std::make_unique<V{}>();
        
        // Initialize trace
        Verilated::traceEverOn(true);
        trace = std::make_unique<VerilatedVcdC>();
        dut->trace(trace.get(), 99);
        trace->open("{}.vcd");
        
        // Initialize signals
        dut->ap_rst_n = 0;
        dut->ap_clk = 0;
        dut->ap_start = 0;
    }}
    
    ~{}Sim() {{
        if (trace) {{
            trace->close();
        }}
        dut->final();
    }}
    
    void clock_tick() {{
        dut->ap_clk = 0;
        dut->eval();
        trace->dump(sim_time++);
        
        dut->ap_clk = 1;
        dut->eval();
        trace->dump(sim_time++);
    }}
    
    void reset() {{
        dut->ap_rst_n = 0;
        for (int i = 0; i < 5; i++) {{
            clock_tick();
        }}
        dut->ap_rst_n = 1;
        clock_tick();
    }}
    
    void start_computation() {{
        dut->ap_start = 1;
        clock_tick();
        dut->ap_start = 0;
    }}
    
    bool is_done() {{
        return dut->ap_done;
    }}
    
    bool is_idle() {{
        return dut->ap_idle;
    }}
    
    // Input setters (these will be generated based on actual inputs)
    void set_input_a(uint32_t value) {{
        dut->a = value;
    }}
    
    void set_input_b(uint32_t value) {{
        dut->b = value;
    }}
    
    // Output getters (these will be generated based on actual outputs)
    uint32_t get_output_result() {{
        return dut->result;
    }}
    
    void run_until_done() {{
        start_computation();
        while (!is_done()) {{
            clock_tick();
            if (sim_time > 1000) {{ // Timeout protection
                std::cerr << "Simulation timeout!" << std::endl;
                break;
            }}
        }}
        clock_tick(); // One more cycle to see the done signal
    }}
}};

// C interface for Rust FFI
extern "C" {{
    void* create_sim() {{
        return new {}Sim();
    }}
    
    void destroy_sim(void* sim) {{
        delete static_cast<{}Sim*>(sim);
    }}
    
    void reset_sim(void* sim) {{
        static_cast<{}Sim*>(sim)->reset();
    }}
    
    void set_input_a_sim(void* sim, uint32_t value) {{
        static_cast<{}Sim*>(sim)->set_input_a(value);
    }}
    
    void set_input_b_sim(void* sim, uint32_t value) {{
        static_cast<{}Sim*>(sim)->set_input_b(value);
    }}
    
    uint32_t get_output_result_sim(void* sim) {{
        return static_cast<{}Sim*>(sim)->get_output_result();
    }}
    
    void run_until_done_sim(void* sim) {{
        static_cast<{}Sim*>(sim)->run_until_done();
    }}
    
    int is_done_sim(void* sim) {{
        return static_cast<{}Sim*>(sim)->is_done() ? 1 : 0;
    }}
}}
"#,
            self.module_name, // V{}.h include
            self.module_name, // V{} class
            self.module_name, // {}Sim class name
            self.module_name, // V{} member
            self.module_name, // {}Sim constructor
            self.module_name, // V{} constructor
            self.module_name, // VCD filename
            self.module_name, // ~{}Sim destructor
            self.module_name, // create_sim return
            self.module_name, // destroy_sim cast
            self.module_name, // reset_sim cast
            self.module_name, // set_input_a_sim cast
            self.module_name, // set_input_b_sim cast
            self.module_name, // get_output_result_sim cast
            self.module_name, // run_until_done_sim cast
            self.module_name, // is_done_sim cast
        );
        
        let cpp_path = self.sim_dir.join("testbench.cpp");
        fs::write(cpp_path, cpp_code)
            .map_err(|e| format!("Failed to write C++ testbench: {}", e))?;
        
        println!("Generated C++ testbench: {}", self.sim_dir.join("testbench.cpp").display());
        
        Ok(())
    }
    
    /// Run Verilator to generate C++ from Verilog
    fn run_verilator(&mut self, verilog_path: &Path) -> Result<(), String> {
        // Get the absolute path, but handle Windows UNC path issues
        let abs_verilog_path = if verilog_path.is_absolute() {
            verilog_path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .join(verilog_path)
        };
        
        // Convert to string and normalize path separators for Verilator
        let verilog_path_str = abs_verilog_path.to_string_lossy()
            .replace("\\", "/")
            .trim_start_matches("//?/")  // Remove Windows UNC prefix if present
            .to_string();
        
        // Set VERILATOR_ROOT environment variable to help Verilator find its files
        let mut cmd = Command::new("verilator");
        
        // Try to set VERILATOR_ROOT if we can find it
        if let Ok(root) = self.find_verilator_root() {
            cmd.env("VERILATOR_ROOT", &root);
            println!("Setting VERILATOR_ROOT to: {}", root);
        }
        
        cmd.arg("--cc")                    // Generate C++
            .arg("--exe")                   // Generate executable
            .arg("--build")                 // Build the executable
            .arg("--trace")                 // Enable VCD tracing
            .arg("-Wall")                   // Enable warnings
            .arg("-Wno-UNUSED")            // Disable unused warnings
            .arg("-Wno-UNDRIVEN")          // Disable undriven warnings
            .arg("-Wno-WIDTHTRUNC")        // Disable width truncation warnings
            .arg("--top-module")
            .arg(&self.module_name)
            .arg("testbench.cpp")          // Use our testbench file
            .arg(&verilog_path_str)        // Use normalized absolute path
            .current_dir(&self.sim_dir);   // Work in sim directory
        
        println!("Running Verilator with Verilog file: {}", verilog_path_str);
        
        let output = cmd.output();
        
        match output {
            Ok(result) => {
                if !result.status.success() {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    return Err(format!("Verilator failed:\nstdout: {}\nstderr: {}", stdout, stderr));
                }
                
                // Set the executable path
                self.set_executable_path();
                
                println!("Verilator compilation successful!");
                println!("Verilated files in: {}", self.sim_dir.join("obj_dir").display());
                Ok(())
            }
            Err(e) => Err(format!("Failed to run Verilator: {}. Make sure Verilator is installed.", e))
        }
    }
    
    /// Find Verilator root directory
    fn find_verilator_root(&self) -> Result<String, String> {
        // Try to get VERILATOR_ROOT from verilator itself
        if let Ok(output) = Command::new("verilator").arg("--getenv").arg("VERILATOR_ROOT").output() {
            if output.status.success() {
                let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !root.is_empty() && std::path::Path::new(&root).exists() {
                    return Ok(root);
                }
            }
        }
        
        // Common locations for Verilator root
        let possible_roots = vec![
            "C:/msys64/mingw64/share/verilator",
            "/mingw64/share/verilator",
            "/usr/share/verilator",
            "/usr/local/share/verilator",
            "/opt/homebrew/share/verilator",
        ];
        
        for root in &possible_roots {
            let root_path = std::path::Path::new(root);
            if root_path.exists() {
                let include_path = root_path.join("include");
                if include_path.exists() {
                    // Check if the key files exist
                    let std_waiver = include_path.join("verilated_std_waiver.vlt");
                    let std_sv = include_path.join("verilated_std.sv");
                    
                    if std_waiver.exists() && std_sv.exists() {
                        return Ok(root.to_string());
                    }
                }
            }
        }
        
        Err("Could not find Verilator root directory".to_string())
    }
    
    /// Compile the generated C++ with proper linking
    fn compile_cpp(&self) -> Result<(), String> {
        // The --build flag in Verilator should handle this,
        // but we can add additional compilation steps here if needed
        Ok(())
    }
    
    /// Set the path to the Verilated executable
    fn set_executable_path(&mut self) {
        self.verilated_executable = Some(self.sim_dir.join("obj_dir").join(format!("V{}", self.module_name)));
    }
    
    /// Get the path to the compiled simulation library
    pub fn get_library_path(&self) -> Option<PathBuf> {
        Some(self.sim_dir.join("obj_dir").join(format!("libV{}.a", self.module_name)))
    }
    
    /// Get the verilog output directory path
    pub fn get_verilog_out_dir(&self) -> &Path {
        &self.verilog_out_dir
    }
    
    /// Get the simulation directory path  
    pub fn get_sim_dir(&self) -> &Path {
        &self.sim_dir
    }
    
    /// Get the Verilator object directory path
    pub fn get_obj_dir(&self) -> PathBuf {
        self.sim_dir.join("obj_dir")
    }
    
    /// Get the module name
    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
}

/// Create a dynamic library for FFI with Rust
pub fn create_shared_library(module_name: &str, sim_dir: &Path) -> Result<PathBuf, String> {
    // Determine the library filename based on platform
    let lib_filename = if cfg!(target_os = "windows") {
        format!("{}_sim.dll", module_name)
    } else if cfg!(target_os = "macos") {
        format!("lib{}_sim.dylib", module_name)
    } else {
        format!("lib{}_sim.so", module_name)
    };
    
    let lib_path = sim_dir.join(&lib_filename);
    let obj_dir = sim_dir.join("obj_dir");
    
    // Check if Verilator generated the necessary files
    let verilated_cpp = obj_dir.join(format!("V{}.cpp", module_name));
    if !verilated_cpp.exists() {
        return Err(format!("Verilated C++ file not found: {}", verilated_cpp.display()));
    }
    
    // Determine compiler and flags based on platform
    let (compiler, args) = if cfg!(target_os = "windows") {
        // Try to use MSVC on Windows
        let mut args = vec![
            "/LD".to_string(), // Create DLL
            "/Fe:".to_string() + &lib_path.to_string_lossy(),
            format!("/I{}", get_verilator_include_dir()?),
        ];
        
        // Add source files
        args.push(verilated_cpp.to_string_lossy().to_string());
        
        // Add optional files if they exist
        let optional_files = vec![
            format!("V{}__Trace__0__Slow.cpp", module_name),
            format!("V{}__Syms.cpp", module_name),
        ];
        
        for file in optional_files {
            let file_path = obj_dir.join(&file);
            if file_path.exists() {
                args.push(file_path.to_string_lossy().to_string());
            }
        }
        
        args.push("testbench.cpp".to_string());
        args.push(format!("{}/verilated.cpp", get_verilator_include_dir()?));
        
        ("cl", args)
    } else {
        // Use GCC/G++ on Unix-like systems
        let mut args = vec![
            "-shared".to_string(),
            "-fPIC".to_string(),
            "-o".to_string(),
            lib_path.to_string_lossy().to_string(),
            "-I".to_string(),
            get_verilator_include_dir()?,
            "-I".to_string(),
            format!("{}/vltstd", get_verilator_include_dir()?),
        ];
        
        // Add source files
        args.push(verilated_cpp.to_string_lossy().to_string());
        
        // Add optional files if they exist
        let optional_files = vec![
            format!("V{}__Trace__0__Slow.cpp", module_name),
            format!("V{}__Syms.cpp", module_name),
        ];
        
        for file in optional_files {
            let file_path = obj_dir.join(&file);
            if file_path.exists() {
                args.push(file_path.to_string_lossy().to_string());
            }
        }
        
        args.push("testbench.cpp".to_string());
        args.push(format!("{}/verilated.cpp", get_verilator_include_dir()?));
        args.push(format!("{}/verilated_vcd_c.cpp", get_verilator_include_dir()?));
        args.push(format!("{}/verilated_threads.cpp", get_verilator_include_dir()?));
        
        ("g++", args)
    };
    
    println!("Creating shared library with {}: {}", compiler, args.join(" "));
    
    let output = Command::new(compiler)
        .args(&args)
        .current_dir(sim_dir)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("Created shared library: {}", lib_path.display());
                Ok(lib_path)
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let stdout = String::from_utf8_lossy(&result.stdout);
                Err(format!("Failed to create shared library:\nStderr: {}\nStdout: {}", stderr, stdout))
            }
        }
        Err(e) => Err(format!("Failed to run {}: {}", compiler, e))
    }
}

/// Get Verilator include directory (platform-specific)
fn get_verilator_include_dir() -> Result<String, String> {
    if cfg!(target_os = "windows") {
        // On Windows, Verilator might be installed via MSYS2, WSL, or Chocolatey
        let possible_paths = vec![
            "C:/msys64/mingw64/share/verilator/include",
            "C:/tools/verilator/include",
            "/usr/share/verilator/include", // WSL
        ];
        
        for path in possible_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }
        
        Err("Verilator include directory not found. Please install Verilator via MSYS2, WSL, or manually set the path.".to_string())
    } else if cfg!(target_os = "macos") {
        // On macOS, check Homebrew location
        let possible_paths = vec![
            "/opt/homebrew/share/verilator/include", // Apple Silicon
            "/usr/local/share/verilator/include",    // Intel
        ];
        
        for path in possible_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }
        
        Err("Verilator include directory not found. Please install with: brew install verilator".to_string())
    } else {
        // Linux
        let possible_paths = vec![
            "/usr/share/verilator/include",
            "/usr/local/share/verilator/include",
        ];
        
        for path in possible_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }
        
        Err("Verilator include directory not found. Please install with: sudo apt install verilator".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::*;
    use crate::ir::lower::*;
    
    #[test]
    fn test_verilator_compilation() {
        // Create a simple adder circuit
        let a = input("a", 32);
        let b = input("b", 32);
        let sum = add(a, b);
        let result = output("result", sum);
        
        let graph = lower_expr_to_graph(&result);
        
        let mut verilator_sim = VerilatorSim::new("test_adder");
        
        // This test will only pass if Verilator is installed
        if let Err(e) = verilator_sim.compile_from_graph(&graph) {
            if e.contains("Failed to run Verilator") {
                println!("Skipping Verilator test - Verilator not installed: {}", e);
                return; // Skip test if Verilator is not available
            } else {
                panic!("Unexpected error: {}", e);
            }
        }
        
        println!("Verilator compilation test passed!");
    }
}
