@echo off
echo ========================================
echo Installing Zweites Gehirn Dev Tools
echo ========================================
echo.

echo Checking for existing installations...
echo.

where rustc >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Rust is already installed
    rustc --version
) else (
    echo [INSTALLING] Rust...
    winget install Rustlang.Rustup
)

echo.
where node >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Node.js is already installed
    node --version
) else (
    echo [INSTALLING] Node.js...
    winget install OpenJS.NodeJS.LTS
)

echo.
where pnpm >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] pnpm is already installed
    pnpm --version
) else (
    echo [INSTALLING] pnpm...
    call npm install -g pnpm
)

echo.
where cargo >nul 2>&1
if %errorlevel% == 0 (
    echo Checking for Tauri CLI...
    cargo tauri --version >nul 2>&1
    if %errorlevel% == 0 (
        echo [OK] Tauri CLI is already installed
    ) else (
        echo [INSTALLING] Tauri CLI... (this takes a few minutes)
        cargo install tauri-cli --locked
    )
)

echo.
echo ========================================
echo Installation Complete!
echo ========================================
echo.
echo IMPORTANT: Close this window and open a NEW terminal, then run:
echo   cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
echo   pnpm install
echo   pnpm tauri dev
echo.
pause
