# Clean Vivado Project Recreation Script
# ====================================
# Use this to recreate the project from scratch if it gets corrupted

puts "🧹 Cleaning and Recreating HFT Vivado Project"
puts "============================================="

# Clean up any existing project
set project_name "hft_fpga_socket_sim"
set project_dir "./vivado_hft_project"

# Close any open project first
catch {close_project -quiet}

# Remove existing project directory if it exists
file delete -force $project_dir

puts "Removed old project directory"

# Wait a moment for filesystem
after 1000

# Recreate project directory
file mkdir $project_dir

puts "Created clean project directory"

# Project settings
set part_name "xcu50-fsvh2104-2-e"

# Create new project
puts "Creating fresh project: $project_name"
create_project $project_name $project_dir -part $part_name -force

puts "✅ Fresh project created successfully"

# Add HFT Verilog source
set hft_verilog_file "../verilog_out/hft_zero_plus.v" 
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
set tb_file "../verilog_out/tb_hft_socket.sv"
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

# Force save the project
puts "Saving project..."
# Vivado auto-saves projects, but we can force update files
update_compile_order -fileset sources_1
update_compile_order -fileset sim_1

puts "🎯 Clean Project Recreation Complete!"
puts ""
puts "NEXT STEPS:"
puts "==========="
puts "1. Run simulation:"
puts "   launch_simulation"
puts ""
puts "2. Wait for:"
puts "   'Socket server listening on port 8888'"
puts ""
puts "3. Connect Python client:"
puts "   cd ../../python"
puts "   python vivado_hft_trading.py"
puts ""
puts "✅ Ready to launch simulation!"
