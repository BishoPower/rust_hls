# Quick Vivado File Bridge Test
# ============================
# Tests the file-based FPGA communication (no socket issues!)

puts "🧪 Testing HFT FPGA File Bridge"
puts "==============================="

# Launch the file-based simulation
puts "🚀 Launching file bridge simulation..."
launch_simulation

puts "✅ File bridge simulation started!"
puts ""
puts "📁 FPGA reads market data from: fpga_bridge/market_data.json"
puts "📁 FPGA writes decisions to: fpga_bridge/fpga_output.json"
puts ""
puts "🐍 Now run Python bridge:"
puts "   cd ../python"
puts "   python fpga_file_bridge.py"
puts ""
puts "🔍 Watch Vivado console for FPGA trading decisions!"
