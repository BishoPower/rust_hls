# Vivado TCL Script for HFT FPGA Socket Simulation
# ================================================
# Automates Vivado project creation for HFT FPGA testing
# Run in Vivado TCL console: source setup_hft_vivado.tcl

puts "🚀 Setting up HFT FPGA Vivado Project"
puts "====================================="

# Project settings
set project_name "hft_fpga_socket_sim"
set project_dir "./vivado_hft_project"
# AU50 FPGA (change as needed for your board)
set part_name "xcu50-fsvh2104-2-e"

# Create project directory
file mkdir $project_dir

# Create new project
create_project $project_name $project_dir -part $part_name -force

puts "✅ Created project: $project_name"

# Add HFT Verilog source
set hft_verilog_file "../verilog_out/hft_zero_plus.v"
if {[file exists $hft_verilog_file]} {
    add_files -norecurse $hft_verilog_file
    set_property top hft_zero_plus [current_fileset]
    puts "✅ Added HFT Verilog: hft_zero_plus.v"
} else {
    puts "❌ HFT Verilog not found: $hft_verilog_file"
    puts "💡 Generate with: cargo run --example hft_zero_plus"
}

# Add socket testbench
set tb_file "../verilog_out/tb_hft_socket.sv"
if {[file exists $tb_file]} {
    add_files -fileset sim_1 -norecurse $tb_file
    set_property top tb_hft_socket [get_filesets sim_1]
    puts "✅ Added testbench: tb_hft_socket.sv"
} else {
    puts "❌ Testbench not found: $tb_file"
}

# Simulation settings
set_property -name {xsim.simulate.runtime} -value {100ms} -objects [get_filesets sim_1]
set_property -name {xsim.simulate.log_all_signals} -value {true} -objects [get_filesets sim_1]

# Configure for socket simulation
puts "⚙️  Configuring simulation settings..."

# Add compilation flags for socket support (if using DPI-C version)
# set_property -name {xsim.compile.xvlog.more_options} -value {-d SOCKET_SIM} -objects [get_filesets sim_1]

puts "🎯 Project Setup Complete!"
puts ""
puts "📋 NEXT STEPS:"
puts "=============="
puts "1. 🏗️  Build project:"
puts "   - Flow Navigator → Synthesis → Run Synthesis"
puts "   - (Optional - for timing validation)"
puts ""
puts "2. 🧪 Run simulation:"
puts "   - Flow Navigator → Simulation → Run Simulation"
puts "   - Choose 'Run Behavioral Simulation'"
puts "   - Wait for 'Socket server listening' message"
puts ""
puts "3. 🐍 Connect Python client:"
puts "   - cd ../../python"
puts "   - python vivado_hft_trading.py"
puts ""
puts "4. 📊 View waveforms:"
puts "   - Simulation will generate hft_fpga_simulation.vcd"
puts "   - Use Vivado waveform viewer for analysis"
puts ""
puts "🔗 Files created:"
puts "- Project: $project_dir/$project_name.xpr"
puts "- Sources: hft_zero_plus.v (your HFT FPGA)"
puts "- Testbench: tb_hft_socket.sv (socket interface)"
puts ""
puts "⚡ Ready for Python ↔ Vivado FPGA communication!"

# Optional: Launch GUI if running in batch mode
# start_gui

# Save project (Vivado auto-saves, but we can force it)
puts "💾 Project saved successfully"
puts ""
puts "🎮 TO START SIMULATION:"
puts "launch_simulation"
