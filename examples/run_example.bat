:
@echo off
echo Building Windows Environment Variables Manager...
cargo build --release
echo.
echo Running the application...
cargo run --release
echo.
echo Application finished.
pause