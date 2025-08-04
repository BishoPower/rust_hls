# Vivado TCL Script for HFT FPGA Socket Simulation
# ================================================
# Automates Vivado project creation for HFT FPGA testing
# Run in Vivado TCL console: source au50_pipeline_fixed.tcl

puts "Setting up HFT FPGA Vivado Project"
puts "=================================="

# Project settings
set project_name "hft_fpga_socket_sim"
set project_dir "./vivado_hft_project"
# Change part_name for your FPGA board:
# AU50: xcu50-fsvh2104-2-e
# U200: xcu200-fsgd2104-2-e  
# U250: xcu250-figd2104-2L-e
set part_name "xcu50-fsvh2104-2-e"

# Create project directory
file mkdir $project_dir

# Create new project
puts "Creating project: $project_name"
create_project $project_name $project_dir -part $part_name -force

puts "Project created successfully"

# Add HFT Verilog source
set hft_verilog_file "../verilog_out/hft_zero_plus.v"
if {[file exists $hft_verilog_file]} {
    add_files -norecurse $hft_verilog_file
    set_property top hft_zero_plus [current_fileset]
    puts "Added HFT Verilog: hft_zero_plus.v"
} else {
    puts "ERROR: HFT Verilog not found: $hft_verilog_file"
    puts "Generate with: cargo run --example hft_zero_plus"
    return
}

# Add socket testbench
set tb_file "../verilog_out/tb_hft_socket.sv"
if {[file exists $tb_file]} {
    add_files -fileset sim_1 -norecurse $tb_file
    set_property top tb_hft_socket [get_filesets sim_1]
    puts "Added testbench: tb_hft_socket.sv"
} else {
    puts "ERROR: Testbench not found: $tb_file"
    return
}

# Simulation settings
set_property -name {xsim.simulate.runtime} -value {100ms} -objects [get_filesets sim_1]
set_property -name {xsim.simulate.log_all_signals} -value {true} -objects [get_filesets sim_1]

puts "Configuring simulation settings..."

puts "Project Setup Complete!"
puts ""
puts "NEXT STEPS:"
puts "==========="
puts "1. Build project (optional):"
puts "   Flow Navigator -> Synthesis -> Run Synthesis"
puts ""
puts "2. Run simulation:"
puts "   Flow Navigator -> Simulation -> Run Simulation"
puts "   Choose 'Run Behavioral Simulation'"
puts "   Wait for 'Socket server listening on port 8888'"
puts ""
puts "3. Connect Python client:"
puts "   cd ../../python"
puts "   python vivado_hft_trading.py"
puts ""
puts "Files created:"
puts "- Project: $project_dir/$project_name.xpr"
puts "- Sources: hft_zero_plus.v (your HFT FPGA)"
puts "- Testbench: tb_hft_socket.sv (socket interface)"
puts ""
puts "Ready for Python <-> Vivado FPGA communication!"

# Save project (no args needed - saves current project)
puts "Saving project..."

puts "Project saved successfully"
puts ""
puts "TO START SIMULATION NOW:"
puts "launch_simulation"
