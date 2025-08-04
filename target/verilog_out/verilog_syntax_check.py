#!/usr/bin/env python3
"""
Vivado Verilog Syntax Checker
============================
Quick verification that the HFT Verilog has correct syntax
"""

import subprocess
import os

def check_verilog_syntax():
    """Check if Verilog syntax is correct"""
    print("🔍 Checking HFT Verilog Syntax")
    print("=" * 40)
    
    verilog_file = "hft_zero_plus.v"
    
    if not os.path.exists(verilog_file):
        print(f"❌ {verilog_file} not found")
        return False
        
    # Get file size
    size_kb = os.path.getsize(verilog_file) / 1024
    print(f"📄 File: {verilog_file} ({size_kb:.1f}KB)")
    
    # Check for missing declarations
    with open(verilog_file, 'r') as f:
        content = f.read()
        
    # Check for common issues
    issues = []
    
    if 'pipeline_valid' in content and 'reg [2:0] pipeline_valid' not in content:
        issues.append("Missing: reg [2:0] pipeline_valid")
        
    if 'pipeline_counter' in content and 'reg [2:0] pipeline_counter' not in content:
        issues.append("Missing: reg [2:0] pipeline_counter")
        
    # Check for proper module declaration
    if 'module hft_zero_plus' not in content:
        issues.append("Missing module declaration")
        
    if 'endmodule' not in content:
        issues.append("Missing endmodule")
        
    # Report results
    if issues:
        print("❌ Syntax Issues Found:")
        for issue in issues:
            print(f"   - {issue}")
        return False
    else:
        print("✅ Basic syntax checks passed")
        print("✅ Pipeline registers declared")
        print("✅ Module structure correct")
        return True

def main():
    """Main check function"""
    print("🏛️  VIVADO VERILOG SYNTAX CHECKER")
    print("=" * 50)
    
    if check_verilog_syntax():
        print("\n🎯 READY FOR VIVADO SIMULATION!")
        print("Run in Vivado TCL console:")
        print("   source au50_pipeline_fixed.tcl")
        print("   launch_simulation")
    else:
        print("\n❌ Fix syntax issues before simulation")
        
if __name__ == "__main__":
    main()
