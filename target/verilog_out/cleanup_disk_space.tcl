# Emergency Disk Space Cleanup for Vivado
# ======================================
# Run this to free up disk space before simulation

puts "🧹 Emergency Disk Space Cleanup"
puts "==============================="

# Close any open simulations first
catch {close_sim -force}

# Remove large simulation files
set cleanup_patterns {
    "*.wdb"
    "*.vcd" 
    "*.saif"
    "*.log"
    "*xsim*"
}

foreach pattern $cleanup_patterns {
    set files [glob -nocomplain $pattern]
    foreach file $files {
        if {[file exists $file]} {
            catch {file delete -force $file}
            puts "🗑️ Deleted: $file"
        }
    }
}

# Clean project directories
set project_dirs [glob -nocomplain "vivado_hft_project*"]
foreach dir $project_dirs {
    if {[file isdirectory $dir]} {
        set sim_dir "$dir/*.sim"
        catch {file delete -force {*}[glob -nocomplain $sim_dir]}
        puts "🗑️ Cleaned simulation files in: $dir"
    }
}

puts "✅ Disk cleanup complete!"
puts "💾 Check disk space: Get-WmiObject Win32_LogicalDisk"
