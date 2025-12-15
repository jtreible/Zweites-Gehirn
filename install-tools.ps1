# Installation script for Zweites Gehirn development environment
# Run this in PowerShell as Administrator

Write-Host "Installing development tools for Zweites Gehirn..." -ForegroundColor Green

# Install Rust
Write-Host "`nInstalling Rust..." -ForegroundColor Yellow
if (!(Get-Command rustc -ErrorAction SilentlyContinue)) {
    Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
    .\rustup-init.exe -y
    Remove-Item rustup-init.exe
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
} else {
    Write-Host "Rust already installed, updating..." -ForegroundColor Cyan
    rustup update
}

# Install Node.js if needed
Write-Host "`nChecking Node.js..." -ForegroundColor Yellow
if (!(Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Node.js LTS via winget..." -ForegroundColor Yellow
    winget install OpenJS.NodeJS.LTS
} else {
    $nodeVersion = node --version
    Write-Host "Node.js already installed: $nodeVersion" -ForegroundColor Cyan
}

# Install pnpm
Write-Host "`nInstalling pnpm..." -ForegroundColor Yellow
npm install -g pnpm

# Install Tauri CLI
Write-Host "`nInstalling Tauri CLI..." -ForegroundColor Yellow
cargo install tauri-cli --locked

Write-Host "`n✓ Installation complete!" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "1. Close and reopen your terminal"
Write-Host "2. Navigate to the project directory"
Write-Host "3. Run: pnpm install"
Write-Host "4. Run: pnpm tauri dev"
