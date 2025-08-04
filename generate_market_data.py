"""
Market Data Generator for FPGA Simulation
Generates static market data files that the simulation reads
"""

import time
import json
from pathlib import Path

def generate_market_data_files():
    """Generate static market data files for simulation"""
    
    # Market data scenarios
    scenarios = [
        {"symbol": "SPY", "bid_price": 500.00, "ask_price": 500.01, "bid_qty": 20000, "ask_qty": 15000, "spread": 0.01},
        {"symbol": "QQQ", "bid_price": 349.50, "ask_price": 349.55, "bid_qty": 5000, "ask_qty": 4500, "spread": 0.05},
        {"symbol": "IWM", "bid_price": 199.98, "ask_price": 200.01, "bid_qty": 3000, "ask_qty": 2800, "spread": 0.03},
    ]
    
    print("📊 Generating Market Data Files for FPGA Simulation")
    print("=" * 60)
    
    # Paths where simulation might look for market data
    target_paths = [
        Path("fpga_bridge"),  # Main location
        Path("target/verilog_out/fpga_bridge"),  # Build output location
    ]
    
    # Create directories and write market data files
    for path in target_paths:
        try:
            # Create directory if it doesn't exist
            path.mkdir(parents=True, exist_ok=True)
            
            # Write default market data (SPY scenario)
            market_data = {
                "timestamp": time.time(),
                "symbol": scenarios[0]["symbol"],
                "bid_price": scenarios[0]["bid_price"],
                "ask_price": scenarios[0]["ask_price"],
                "bid_qty": scenarios[0]["bid_qty"],
                "ask_qty": scenarios[0]["ask_qty"],
                "spread": scenarios[0]["spread"]
            }
            
            market_file = path / "market_data.json"
            with open(market_file, 'w') as f:
                json.dump(market_data, f, indent=2)
            
            print(f"✅ Created: {market_file}")
            print(f"   📈 {market_data['symbol']}: ${market_data['bid_price']:.2f}/${market_data['ask_price']:.2f}")
            
            # Create scenario-specific files
            for scenario in scenarios:
                scenario_data = {
                    "timestamp": time.time(),
                    "symbol": scenario["symbol"],
                    "bid_price": scenario["bid_price"],
                    "ask_price": scenario["ask_price"],
                    "bid_qty": scenario["bid_qty"],
                    "ask_qty": scenario["ask_qty"],
                    "spread": scenario["spread"]
                }
                
                scenario_file = path / f"market_data_{scenario['symbol'].lower()}.json"
                with open(scenario_file, 'w') as f:
                    json.dump(scenario_data, f, indent=2)
                
        except Exception as e:
            print(f"⚠️  Could not create {path}: {e}")
    
    print("\n🎯 Market Data Generation Complete!")
    print("📁 Files ready for FPGA simulation")
    print("🚀 Run: vivado -mode batch -source vivado_hft_continuous.tcl")

if __name__ == "__main__":
    generate_market_data_files()
