# Manual Installation Guide

If the automated script fails, follow these steps:

## Step 1: Install Rust (5 minutes)

**Option A: Using winget**
```powershell
winget install Rustlang.Rustup
```

**Option B: Direct download**
1. Go to https://rustup.rs/
2. Download `rustup-init.exe`
3. Run it and follow the prompts
4. Choose "1) Proceed with installation (default)"

**Verify:**
```powershell
# Close and reopen PowerShell, then:
rustc --version
cargo --version
```

You should see version numbers like `rustc 1.92.0`.

---

## Step 2: Install Node.js (3 minutes)

**Option A: Using winget**
```powershell
winget install OpenJS.NodeJS.LTS
```

**Option B: Direct download**
1. Go to https://nodejs.org/
2. Download the LTS version (Windows Installer)
3. Run the installer
4. Accept all defaults

**Verify:**
```powershell
# Close and reopen PowerShell, then:
node --version
npm --version
```

You should see version numbers like `v20.x.x` and `10.x.x`.

---

## Step 3: Install pnpm (1 minute)

```powershell
npm install -g pnpm
```

**Verify:**
```powershell
pnpm --version
```

You should see a version number like `8.x.x`.

---

## Step 4: Install Tauri CLI (2-3 minutes)

```powershell
cargo install tauri-cli --locked
```

This will take a few minutes to compile.

**Verify:**
```powershell
cargo tauri --version
```

---

## Step 5: Install Project Dependencies (2 minutes)

```powershell
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
pnpm install
```

You should see it downloading packages. This creates a `node_modules` folder.

---

## Step 6: Launch the App! (3-5 minutes first time)

```powershell
pnpm tauri dev
```

**First launch notes:**
- Takes 3-5 minutes (compiling Rust dependencies)
- Future launches are much faster (< 30 seconds)
- A window will open with your app

---

## Troubleshooting

### "winget: command not found"
- Update Windows: Settings → Update & Security → Check for updates
- Or use Option B (direct download) for each tool

### "Permission denied" errors
- Run PowerShell as Administrator
- Right-click PowerShell → "Run as Administrator"

### Rust installation asks questions
- Just press Enter for all prompts (accept defaults)

### "Failed to compile" errors
If Rust compilation fails:
```powershell
rustup update
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
cd src-tauri
cargo clean
cd ..
pnpm tauri dev
```

### Node.js already installed but old version
```powershell
# Uninstall old version first via Windows Settings
# Then install latest LTS
winget install OpenJS.NodeJS.LTS
```

---

## Quick Check: What's Already Installed?

Run these commands to see what you already have:

```powershell
rustc --version      # Rust compiler
cargo --version      # Rust package manager
node --version       # Node.js
npm --version        # Node package manager
pnpm --version       # pnpm (might not exist yet)
```

If any command returns a version number, that tool is already installed!

---

## Expected Timeline

- Rust: 5 minutes
- Node.js: 3 minutes
- pnpm: 1 minute
- Tauri CLI: 3 minutes
- Dependencies: 2 minutes
- First app launch: 5 minutes

**Total: ~20 minutes for first-time setup**

---

## When You're Ready to Continue

After everything is installed, just run:

```powershell
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
pnpm tauri dev
```

The app will compile and launch automatically!
