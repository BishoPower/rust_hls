# Run Enhanced FPGA Simulation with Debugging
puts "🚀 Running Enhanced FPGA Simulation"
puts "==================================="

# Open existing project
open_project vivado_hft_file_project/hft_fpga_file_sim.xpr

# Update simulation files
update_compile_order -fileset sim_1

# Run longer simulation (500ms instead of 100ms) to see FPGA completion
set_property -name {xsim.simulate.runtime} -value {500ms} -objects [get_filesets sim_1]

# Launch simulation
launch_simulation

# Run simulation for 500ms
run 500ms

puts "✅ Enhanced simulation complete!"
puts "📊 Check console output for:"
puts "   🔍 FPGA Status updates"
puts "   🚀 Triggered FPGA processing"
puts "   🔄 FPGA Output Changes"
puts "   ⚡ FPGA Decisions"
