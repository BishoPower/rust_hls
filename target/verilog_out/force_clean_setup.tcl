# Force Clean Vivado Project Script
# =================================
# Handles locked/in-use simulation directories

puts "🔄 Force Cleaning Locked Vivado Project"
puts "======================================="

# First, try to close any running simulations
puts "Closing any running simulations..."
catch {close_sim -quiet}

# Close any open project
puts "Closing current project..."
catch {close_project -quiet}

# Wait a moment for processes to release
puts "Waiting for processes to release files..."
after 2000

# Project settings
set project_name "hft_fpga_socket_sim"
set project_dir "./vivado_hft_project_clean"
set part_name "xcu50-fsvh2104-2-e"

# Use a new directory name to avoid locked files
puts "Using clean directory: $project_dir"

# Create project in new directory
puts "Creating fresh project: $project_name"
create_project $project_name $project_dir -part $part_name -force

puts "✅ Fresh project created successfully"

# Add HFT Verilog source
set hft_verilog_file "hft_zero_plus.v"
if {[file exists $hft_verilog_file]} {
    add_files -norecurse $hft_verilog_file
    set_property top hft_zero_plus [current_fileset]
    puts "✅ Added HFT Verilog: hft_zero_plus.v"
} else {
    puts "❌ ERROR: HFT Verilog not found: $hft_verilog_file"
    puts "💡 Generate with: cargo run --example hft_zero_plus"
    return
}

# Add socket testbench
set tb_file "tb_hft_socket.sv"
if {[file exists $tb_file]} {
    add_files -fileset sim_1 -norecurse $tb_file
    set_property top tb_hft_socket [get_filesets sim_1]
    puts "✅ Added testbench: tb_hft_socket.sv"
} else {
    puts "❌ ERROR: Testbench not found: $tb_file"
    return
}

# Simulation settings
set_property -name {xsim.simulate.runtime} -value {100ms} -objects [get_filesets sim_1]
set_property -name {xsim.simulate.log_all_signals} -value {true} -objects [get_filesets sim_1]

puts "✅ Simulation settings configured"

# Update compile order
update_compile_order -fileset sources_1
update_compile_order -fileset sim_1

puts "🎯 Force Clean Project Setup Complete!"
puts ""
puts "NEW PROJECT LOCATION:"
puts "Project: $project_dir/$project_name.xpr"
puts ""
puts "NEXT STEPS:"
puts "==========="
puts "1. Launch simulation:"
puts "   launch_simulation"
puts ""
puts "2. Wait for socket server:"
puts "   'Socket server listening on port 8888'"
puts ""
puts "3. Connect Python client:"
puts "   cd ../../python"
puts "   python vivado_hft_trading.py"
puts ""
puts "✅ Ready to launch simulation!"
