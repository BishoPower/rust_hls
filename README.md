# Rust HLS FPGA Trading System

A high-level synthesis framework for FPGA development focused on high-frequency trading applications.

## Overview

This project provides a complete pipeline from Rust code to FPGA implementation, specifically optimized for low-latency trading applications. The system generates Verilog HDL from Rust descriptions and integrates with Vivado for simulation and synthesis.

## Core Components

### Essential Files

- **`src/`** - Core Rust HLS library and trading modules
- **`tb_hft_continuous_bridge.sv`** - SystemVerilog testbench for FPGA simulation
- **`generate_market_data.py`** - Market data file generator (static files)
- **`vivado_hft_continuous.tcl`** - Vivado batch simulation script
- **`vivado_gui_setup.tcl`** - Vivado GUI setup script
- **`fpga_bridge/`** - Communication bridge between Python and FPGA

### Python Integration

- **`generate_market_data.py`** - Market data file generator (uses only standard libraries)
- **`python/requirements.txt`** - Dependencies (none required)
- **`fpga_bridge/`** - Single communication bridge directory

### Generated Output

- **`target/verilog_out/`** - Generated Verilog modules
- **`target/pipeline_out/`** - Pipeline optimization outputs

## Quick Start

1. **Setup Environment**

   ```bash
   cargo build
   # No Python dependencies needed!
   ```

2. **Run FPGA Simulation (Batch Mode)**

   ```bash
   python generate_market_data.py
   vivado -mode batch -source vivado_hft_continuous.tcl
   ```

3. **Run FPGA Simulation (GUI Mode)**
   ```bash
   vivado -mode gui -source vivado_gui_setup.tcl
   ```

## System Architecture

The system operates as a simple, efficient trading pipeline:

1. **Market Data Generation** - `generate_market_data.py` creates static market data files
2. **File Communication** - JSON files bridge Python and Vivado simulation
3. **FPGA Processing** - HFT module processes market data in hardware
4. **Trading Decisions** - System outputs buy/sell decisions with microsecond latency

## Trading Scenarios

The system includes multiple market scenarios:

- **SPY** - S&P 500 ETF with tight spreads ($500.00/$500.01)
- **QQQ** - NASDAQ ETF with moderate volatility ($349.50/$349.55)
- **IWM** - Russell 2000 ETF with wider spreads ($199.98/$200.01)

## Requirements

- **Rust** 1.70+ with Cargo
- **Python** 3.x (standard libraries only)
- **Vivado** 2022.2+ for FPGA simulation and synthesis
- **Windows** PowerShell environment

## License

See LICENSE file for details.
