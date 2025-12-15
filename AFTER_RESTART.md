# 🚀 After Restart - Quick Start Guide

## ✅ What You've Already Done
- ✅ Installed Rust
- ✅ Installed Node.js and pnpm
- ✅ Installed project dependencies (`pnpm install`)
- ✅ Installed Visual Studio C++ Build Tools
- ✅ Restarted computer

## 🎯 Next Steps (Should Work Now!)

### 1. Open PowerShell
- Press `Windows + X`
- Select "Windows PowerShell" or "Terminal"

### 2. Navigate to Project
```powershell
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
```

### 3. Launch the App
```powershell
pnpm tauri dev
```

**Expected behavior:**
- ⏱️ First compilation takes **3-5 minutes** (this is normal!)
- You'll see lots of "Compiling..." messages
- Eventually, a window will open with your app
- Future launches will be much faster (< 30 seconds)

---

## 🎉 When the App Opens

You should see:
- **"Zweites Gehirn"** heading
- **"Your ADHD-Friendly Second Brain"** subtitle
- An input field that says "Quick add a task..."
- A "Your Tasks" section (empty at first)

### Try It Out!
1. **Add a task**: Type something and press Enter
2. **Complete a task**: Click the ✓ button
3. **Delete a task**: Click the 🗑️ button
4. **Filter tasks**: Click "To Do" or "Completed" buttons

---

## 🐛 If You Still Get Errors

### Error: "cannot open file 'msvcrt.lib'"
The VS C++ tools might not be fully configured. Try:
```powershell
# Switch to GNU toolchain
rustup default stable-x86_64-pc-windows-gnu
```

Then run `pnpm tauri dev` again.

### Error: "command not found"
Open a **new** PowerShell window (the restart should have updated PATH).

### Error: Other compilation errors
```powershell
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn"
cd src-tauri
cargo clean
cd ..
pnpm tauri dev
```

---

## 📚 Once the App Works

Check out these files:
- **README.md** - Full project documentation
- **PROJECT_STATUS.md** - What's been built
- **Plan** (`.claude/plans/`) - Complete development roadmap

### Development Commands
```powershell
pnpm tauri dev     # Run with hot-reload (for development)
pnpm tauri build   # Build production version (creates installer)
pnpm format        # Format code
pnpm lint          # Check code quality
```

---

## 🎯 Your Project is Ready!

After the app launches successfully, you have:
- ✅ A working ADHD-friendly task manager
- ✅ Complete Phase 0 & Phase 1 MVP
- ✅ Foundation for AI features (Phase 3+)
- ✅ Multi-user architecture ready (Phase 8+)

**Next**: Add AI brain dump, Kanban board, or any features from the roadmap!

---

## 💡 Quick Reference

**Project Location:**
```
C:\Users\Operation1\Documents\GitHub\Zweites Gehirn
```

**Quick Launch (save as a shortcut):**
```powershell
cd "C:\Users\Operation1\Documents\GitHub\Zweites Gehirn" && pnpm tauri dev
```

**Database Location:**
```
C:\Users\Operation1\AppData\Roaming\zweites-gehirn\db\zweites_gehirn.db
```

---

**Questions?** Check the troubleshooting section in SETUP.md or README.md!

Good luck! 🚀
