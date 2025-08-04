# Vivado TCL Script for HFT FPGA File Bridge Simulation
# ====================================================
# File-based communication (no socket limitations)
# Works reliably with Python fpga_file_bridge.py

puts "🚀 Setting up HFT FPGA File Bridge Project"
puts "=========================================="

# Project settings
set project_name "hft_fpga_file_sim"
set project_dir "./vivado_hft_file_project"
set part_name "xcu50-fsvh2104-2-e"

# Create project directory
file mkdir $project_dir

# Create new project
create_project $project_name $project_dir -part $part_name -force

puts "✅ Created project: $project_name"

# Add HFT Verilog source
set hft_verilog_file "hft_zero_plus.v"
if {[file exists $hft_verilog_file]} {
    add_files -norecurse $hft_verilog_file
    puts "✅ Added HFT FPGA source: $hft_verilog_file"
} else {
    puts "❌ ERROR: HFT Verilog file not found: $hft_verilog_file"
    puts "💡 Make sure you're in the verilog_out directory"
}

# Add file-based testbench
set testbench_file "tb_hft_file_bridge.sv"
if {[file exists $testbench_file]} {
    add_files -fileset sim_1 -norecurse $testbench_file
    puts "✅ Added file bridge testbench: $testbench_file"
} else {
    puts "❌ ERROR: Testbench file not found: $testbench_file"
}

# Set testbench as top module
set_property top tb_hft_file_bridge [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]

# Configure simulation settings for file I/O
set_property -name {xsim.simulate.runtime} -value {100ms} -objects [get_filesets sim_1]
set_property -name {xsim.simulate.log_all_signals} -value {false} -objects [get_filesets sim_1]

# Create fpga_bridge directory in simulation
file mkdir "fpga_bridge"

puts "✅ Project setup complete!"
puts ""
puts "🚀 Next steps:"
puts "1. launch_simulation"
puts "2. Run Python: cd ../python && python fpga_file_bridge.py"
puts "3. Watch FPGA process market data via files!"
puts ""
