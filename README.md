# Rust HLS - High-Level Synthesis Compiler

A modern High-Level Synthesis (HLS) compiler written in Rust that generates optimized Verilog code for FPGA development. Specifically designed and optimized for AMD Alveo U50 FPGAs and Vivado 2025.

## Features

- **Pipeline Generation**: Automatic pipeline scheduling with configurable initiation interval (II) and depth
- **DSP Optimization**: Targets AMD Alveo U50's DSP48E2 blocks for efficient arithmetic operations
- **Clean Verilog Output**: Generates synthesis-ready Verilog with proper timing constraints
- **Multiple Backends**: Support for Verilator simulation and Vivado synthesis
- **Type-Safe IR**: Strongly typed intermediate representation for reliable transformations

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Optional: Vivado 2025.x for synthesis and simulation
- Optional: Verilator for fast simulation

### Installation

```bash
git clone https://github.com/yourusername/rust_hls.git
cd rust_hls
cargo build --release
```

### Basic Usage

Run the main compiler to see available options:

```bash
cargo run
```

Generate a pipelined MAC (Multiply-Accumulate) example:

```bash
cargo run --example pipelined_mac
```

This creates optimized Verilog in `target/verilog_out/pipelined_mac.v`.

## Example: Pipelined MAC

The included MAC example demonstrates a 5-stage pipeline implementing:

```
result = (a * b) + (c * d) + e
```

### Generated Pipeline Structure

```
Stage 1-2: Parallel multiplications (a*b) and (c*d)
Stage 3:   Addition of multiplication results
Stage 4:   Final accumulation with input e
Stage 5:   Output registration
```

### Key Features

- **Initiation Interval**: II=1 (new input every cycle)
- **Latency**: 5 cycles for full pipeline
- **Throughput**: One result per clock cycle after initial latency
- **Resource Usage**: 2 DSP48E2 blocks, minimal logic resources

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # CLI interface
├── dsl/                # Domain-Specific Language
│   ├── ast.rs          # Abstract syntax tree
│   └── hls.rs          # HLS-specific constructs
├── ir/                 # Intermediate Representation
│   ├── graph.rs        # Computation graph and operations
│   └── lower.rs        # IR lowering passes
├── passes/             # Optimization passes
│   └── pipeline.rs     # Pipeline scheduling algorithm
└── backend/            # Code generation
    ├── verilog.rs      # Verilog generation
    ├── testbench.rs    # Testbench generation
    └── verilator.rs    # Verilator integration

examples/
└── pipelined_mac.rs    # Complete MAC pipeline example

target/
├── verilog_out/        # Generated Verilog files
└── sim/               # Simulation artifacts
```

## Creating Custom Pipelines

### 1. Basic Graph Creation

```rust
use rust_hls::ir::graph::Graph;
use rust_hls::passes::pipeline::PipelineScheduler;
use rust_hls::backend::verilog::generate_verilog_module;

let mut graph = Graph::new();

// Create inputs
let a = graph.add_node_with_output(Operation::Load("a".to_string()));
let b = graph.add_node_with_output(Operation::Load("b".to_string()));

// Add operations
let result = graph.add_node_with_output(Operation::Add(a, b));

// Create output
graph.add_node(Operation::Store("result".to_string(), result));
```

### 2. Enable Pipelining

```rust
// Configure pipeline: II=1, depth=3, no unrolling
graph.enable_pipeline(1, 3, 1);

// Schedule the pipeline
let mut scheduler = PipelineScheduler::new();
scheduler.schedule_pipeline(&mut graph)?;
```

### 3. Generate Verilog

```rust
let verilog = generate_verilog_module(&graph, "my_module");
std::fs::write("output.v", verilog)?;
```

## Supported Operations

| Operation   | Verilog  | Latency   | Notes                         |
| ----------- | -------- | --------- | ----------------------------- |
| `Add(a, b)` | `a + b`  | 1 cycle   | Uses fast carry chains        |
| `Sub(a, b)` | `a - b`  | 1 cycle   | Uses fast carry chains        |
| `Mul(a, b)` | `a * b`  | 3 cycles  | Maps to DSP48E2 blocks        |
| `Div(a, b)` | `a / b`  | 18 cycles | Implements restoring division |
| `And(a, b)` | `a & b`  | 1 cycle   | Bitwise AND                   |
| `Or(a, b)`  | `a \| b` | 1 cycle   | Bitwise OR                    |
| `Not(a)`    | `~a`     | 1 cycle   | Bitwise NOT                   |

## Simulation and Verification

### Using Vivado

1. Open Vivado and create a new project
2. Add the generated `.v` file as a source
3. Create a testbench module
4. Run behavioral simulation
5. Synthesize for Alveo U50 target

### Using Verilator (if available)

The compiler includes Verilator integration for fast simulation:

```bash
# Automatic testbench generation and simulation
# (Work in progress - currently manual testbench required)
```

## Performance Characteristics

### Resource Usage (Typical MAC Pipeline)

- **DSP48E2 Blocks**: 2 (for multiplications)
- **Block RAM**: 0 (register-based design)
- **LUTs**: ~50 (control logic and small additions)
- **Flip-Flops**: ~200 (pipeline registers)

### Timing (Alveo U50 @ 250 MHz)

- **Clock Period**: 4.0 ns
- **Pipeline Latency**: 5 clock cycles (20 ns)
- **Throughput**: 250 M operations/second
- **Power**: Estimated 2-3W for MAC pipeline

## Development Status

This project is actively developed and currently supports:

- ✅ Basic arithmetic operations
- ✅ Pipeline scheduling and optimization
- ✅ Verilog code generation for Alveo U50
- ✅ DSP48E2 inference and optimization
- ⚠️ Verilator integration (basic support)
- 🚧 Advanced control flow (loops, conditionals)
- 🚧 Memory interface generation
- 🚧 Multi-clock domain support

## Contributing

Contributions are welcome! Areas of particular interest:

1. **Memory Controllers**: AXI4 and AXI4-Stream interfaces
2. **Control Flow**: While loops, if/else statements
3. **Optimization**: Dead code elimination, constant propagation
4. **Testing**: More comprehensive verification suites

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by modern HLS tools like Vitis HLS and Intel HLS Compiler
- Built for the AMD Alveo U50 acceleration platform
- Designed for integration with Vivado 2025.x toolchain
