# Setup Guide for Zweites Gehirn

## Quick Start Steps

### 1. Install Development Tools

**Option A: Automated (Recommended)**

Run in PowerShell as Administrator:
\`\`\`powershell
.\install-tools.ps1
\`\`\`

**Option B: Manual Installation**

1. **Install Rust**:
   - Visit https://rustup.rs/
   - Or run: `winget install Rustlang.Rustup`
   - Verify: `rustc --version`

2. **Install Node.js** (v18+):
   - Visit https://nodejs.org/
   - Or run: `winget install OpenJS.NodeJS.LTS`
   - Verify: `node --version`

3. **Install pnpm**:
   \`\`\`bash
   npm install -g pnpm
   \`\`\`

4. **Install Tauri CLI**:
   \`\`\`bash
   cargo install tauri-cli --locked
   \`\`\`

### 2. Install Project Dependencies

\`\`\`bash
pnpm install
\`\`\`

This will install all frontend dependencies including:
- Svelte & SvelteKit
- TypeScript
- Tauri API bindings
- ESLint & Prettier

### 3. Run the App

\`\`\`bash
pnpm tauri dev
\`\`\`

The app will:
- Compile the Rust backend
- Start the Vite dev server
- Launch the desktop application
- Enable hot-reload for both frontend and backend

**First Launch**: The initial build takes 3-5 minutes as it compiles all Rust dependencies. Subsequent launches are much faster (< 30 seconds).

### 4. Verify Everything Works

Once the app launches, you should see:
- ✅ "Zweites Gehirn" heading
- ✅ Quick capture input field
- ✅ Task list (empty initially)

Try adding a task:
1. Type a task title in the input field
2. Press Enter or click "+ Add"
3. The task appears in the list below
4. Click ✓ to complete or 🗑️ to delete

## Troubleshooting

### "rustc: command not found"
- Restart your terminal after installing Rust
- On Windows, you may need to restart your computer
- Verify installation: `rustc --version`

### "pnpm: command not found"
- Ensure Node.js is installed first
- Run `npm install -g pnpm` again
- Restart your terminal

### Compilation errors
- Update Rust: `rustup update`
- Clean build artifacts: `cd src-tauri && cargo clean`
- Try again: `pnpm tauri dev`

### Database errors
The app will automatically create the database on first launch at:
- **Windows**: `%APPDATA%\zweites-gehirn\db\zweites_gehirn.db`

If you encounter database issues:
1. Delete the database folder
2. Restart the app (it will recreate the database)

### Port already in use
If you see "port 1420 is already in use":
1. Kill any existing processes
2. Run `pnpm tauri dev` again

## Development Workflow

### Making Changes

**Frontend (Svelte)**:
- Edit files in `src/`
- Changes auto-reload in the app
- Check browser console for errors

**Backend (Rust)**:
- Edit files in `src-tauri/src/`
- Changes trigger automatic recompile
- Check terminal for compilation errors

### Running Tests

\`\`\`bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests (when added)
pnpm test
\`\`\`

### Code Quality

\`\`\`bash
# Check TypeScript types
pnpm check

# Lint code
pnpm lint

# Format code
pnpm format

# Check Rust formatting
cd src-tauri
cargo fmt --check
\`\`\`

### Building for Production

\`\`\`bash
pnpm tauri build
\`\`\`

Output location:
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Installer**: `zweites-gehirn_0.1.0_x64_en-US.msi`

## Project Structure

\`\`\`
zweites-gehirn/
├── src/                          # Frontend (Svelte)
│   ├── lib/
│   │   ├── api/                 # Tauri API wrappers
│   │   ├── components/          # Svelte components
│   │   ├── stores/              # State management
│   │   └── types/               # TypeScript types
│   └── routes/                  # Pages
│
├── src-tauri/                    # Backend (Rust)
│   └── src/
│       ├── main.rs              # Entry point
│       ├── commands/            # Tauri commands
│       ├── services/            # Business logic
│       └── db/                  # Database
│
├── package.json                  # Frontend dependencies
├── Cargo.toml                    # Rust workspace
├── tauri.conf.json              # Tauri configuration
└── vite.config.ts               # Vite configuration
\`\`\`

## What's Next?

After setup, you can:

1. **Explore the code**:
   - Check out the Rust backend in `src-tauri/src/`
   - Look at Svelte components in `src/lib/components/`

2. **Add features**:
   - See the [PLAN.md](../../../.claude/plans/lively-frolicking-teacup.md) for the roadmap
   - Phase 2 focuses on Kanban boards and visual enhancements

3. **Customize**:
   - Modify colors in `src/app.css`
   - Add new Tauri commands in `src-tauri/src/commands/`

## Getting Help

- **Documentation**: See [README.md](README.md)
- **Issues**: Open an issue on GitHub
- **Plan**: See the detailed plan in `.claude/plans/`

---

Happy coding! 🎉
