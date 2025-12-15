@echo off
echo ========================================
echo Checking Development Tools
echo ========================================
echo.

echo Checking Rust...
where rustc >nul 2>&1
if %errorlevel% == 0 (
    echo [FOUND] Rust compiler
    rustc --version
    cargo --version
) else (
    echo [NOT FOUND] Rust is not installed
    echo Install from: https://rustup.rs/
)

echo.
echo Checking Node.js...
where node >nul 2>&1
if %errorlevel% == 0 (
    echo [FOUND] Node.js
    node --version
    npm --version
) else (
    echo [NOT FOUND] Node.js is not installed
    echo Install from: https://nodejs.org/
)

echo.
echo Checking pnpm...
where pnpm >nul 2>&1
if %errorlevel% == 0 (
    echo [FOUND] pnpm
    pnpm --version
) else (
    echo [NOT FOUND] pnpm is not installed
    echo Run: npm install -g pnpm
)

echo.
echo Checking Tauri CLI...
cargo tauri --version >nul 2>&1
if %errorlevel% == 0 (
    echo [FOUND] Tauri CLI
    cargo tauri --version
) else (
    echo [NOT FOUND] Tauri CLI is not installed
    echo Run: cargo install tauri-cli --locked
)

echo.
echo Checking winget...
where winget >nul 2>&1
if %errorlevel% == 0 (
    echo [FOUND] winget (Windows Package Manager)
) else (
    echo [NOT FOUND] winget not available
    echo Update Windows or install manually
)

echo.
echo ========================================
echo Summary
echo ========================================
echo.
echo If all tools show [FOUND], you can run:
echo   pnpm install
echo   pnpm tauri dev
echo.
echo If any show [NOT FOUND], install them using:
echo   install-tools.bat   (automated)
echo   OR see INSTALL_MANUAL.md for step-by-step
echo.
pause
