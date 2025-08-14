@echo off
echo Testing Windows Environment Variables Manager
echo ============================================
echo.
echo Starting the application...
echo.
echo Features to test:
echo 1. Basic environment variable viewing
echo 2. Search and filter functionality
echo 3. Add/Edit/Delete variables
echo 4. Batch operations mode
echo 5. Settings panel with auto-refresh toggle
echo 6. Manual environment refresh
echo.
echo Press any key to start the application...
pause
cd /d "%~dp0.."
cargo run --release