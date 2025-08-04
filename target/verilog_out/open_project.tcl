# Simple Enhanced Simulation Script
puts "🚀 Opening HFT Project for Enhanced Simulation"

# Open existing project  
open_project vivado_hft_file_project/hft_fpga_file_sim.xpr

# Update files
update_compile_order -fileset sim_1

# Set longer simulation time for debugging
set_property -name {xsim.simulate.runtime} -value {1000ms} -objects [get_filesets sim_1]

# Launch simulation
launch_simulation

puts "✅ Project opened - run simulation manually to see enhanced debugging"
puts "💡 In TCL Console, type: run 1000ms"
