# Vivado Project Diagnosis Script
# ==============================
# Check for common project issues

puts "🔍 Diagnosing Vivado Project Issues"
puts "===================================="

set project_name "hft_fpga_socket_sim"
set project_dir "./vivado_hft_project"
set project_file "$project_dir/$project_name.xpr"

# Check if project file exists
if {[file exists $project_file]} {
    puts "📄 Project file exists: $project_file"
    
    # Check file size
    set file_size [file size $project_file]
    puts "📊 Project file size: $file_size bytes"
    
    if {$file_size < 100} {
        puts "❌ Project file is too small - likely corrupted"
        puts "💡 Recommendation: Recreate project"
    }
} else {
    puts "❌ Project file not found: $project_file"
    puts "💡 Recommendation: Create new project"
}

# Check source files
set hft_verilog "../verilog_out/hft_zero_plus.v"
set testbench "../verilog_out/tb_hft_socket.sv"

if {[file exists $hft_verilog]} {
    set verilog_size [expr [file size $hft_verilog] / 1024]
    puts "✅ HFT Verilog exists: ${verilog_size}KB"
} else {
    puts "❌ HFT Verilog missing: $hft_verilog"
}

if {[file exists $testbench]} {
    puts "✅ Testbench exists: $testbench"
} else {
    puts "❌ Testbench missing: $testbench"
}

# Check if any project is currently open
if {[catch {current_project}]} {
    puts "📂 No project currently open"
} else {
    set current_proj [current_project]
    puts "📂 Current project: $current_proj"
}

puts ""
puts "🔧 SOLUTION:"
puts "============"
puts "The project file appears corrupted."
puts "Run this command to recreate it:"
puts "   source clean_project_setup.tcl"
