@echo off
REM Windows Force Cleanup for Vivado Project
REM =======================================

echo 🧹 Windows Force Cleanup for Vivado Project
echo ==========================================

echo Killing any Vivado processes...
taskkill /F /IM vivado.exe /T 2>nul
taskkill /F /IM xsim.exe /T 2>nul
taskkill /F /IM xelab.exe /T 2>nul
taskkill /F /IM xvlog.exe /T 2>nul

echo Waiting for processes to terminate...
timeout /T 3 /NOBREAK >nul

echo Removing locked project directory...
rmdir /S /Q vivado_hft_project 2>nul

echo ✅ Windows cleanup complete!
echo.
echo NEXT STEPS:
echo ===========
echo 1. Restart Vivado
echo 2. Run: source force_clean_setup.tcl
echo 3. Run: launch_simulation
echo.
pause
