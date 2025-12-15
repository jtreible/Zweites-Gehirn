# Zweites Gehirn 🧠

An ADHD-friendly "Second Brain" task management application built with Tauri, Rust, and Svelte.

## Features

### Phase 1 (MVP) ✅
- ✅ **Quick Capture**: Add tasks instantly with keyboard shortcut (Enter)
- ✅ **Task List**: View all your tasks with filtering
- ✅ **Task Management**: Complete, delete, and organize tasks
- ✅ **Local Storage**: Fast SQLite database with full-text search
- ✅ **ADHD-Friendly**: Clean, focused interface without distractions

### Coming Soon
- 🌟 **AI Brain Dump**: Transform scattered thoughts into organized tasks
- 📊 **Kanban Board**: Visual task organization with drag-and-drop
- 🎯 **Smart Prioritization**: AI-powered task suggestions
- ⏰ **Time Tracking**: Pomodoro timer and time estimates
- 📈 **Progress Tracking**: Streaks, XP, and achievements
- 📅 **Calendar Integration**: Sync with Google Calendar, Outlook, iCloud
- ✉️ **Email Integration**: Create tasks from emails
- 🎤 **Meeting Notes**: AI-powered transcription and action items

## Prerequisites

Before you begin, ensure you have the following installed:

### Required
- **Rust** (latest stable) - [Install from rustup.rs](https://rustup.rs/)
- **Node.js** (v18 or later) - [Install from nodejs.org](https://nodejs.org/)
- **pnpm** - Install after Node.js: `npm install -g pnpm`

### Recommended
- **VS Code** with extensions:
  - Rust Analyzer
  - Svelte for VS Code
  - ESLint
  - Prettier

## Quick Start

### 1. Install Development Tools (Windows)

Run the installation script in PowerShell as Administrator:

\`\`\`powershell
.\install-tools.ps1
\`\`\`

Or install manually:

\`\`\`powershell
# Install Rust
winget install Rustlang.Rustup

# Install Node.js
winget install OpenJS.NodeJS.LTS

# Install pnpm
npm install -g pnpm

# Install Tauri CLI
cargo install tauri-cli --locked
\`\`\`

### 2. Install Dependencies

\`\`\`bash
pnpm install
\`\`\`

### 3. Run Development Server

\`\`\`bash
pnpm tauri dev
\`\`\`

The app will launch with hot-reload enabled. Changes to frontend or backend will automatically rebuild.

### 4. Build for Production

\`\`\`bash
pnpm tauri build
\`\`\`

The installer will be created in `src-tauri/target/release/bundle/`

## Project Structure

\`\`\`
zweites-gehirn/
├── src/                       # Svelte frontend
│   ├── lib/
│   │   ├── api/              # Tauri command wrappers
│   │   ├── components/       # Svelte components
│   │   ├── stores/           # State management
│   │   └── types/            # TypeScript types
│   └── routes/               # SvelteKit pages
│
├── src-tauri/                # Rust backend
│   └── src/
│       ├── commands/         # Tauri IPC handlers
│       ├── services/         # Business logic
│       ├── db/              # Database layer
│       │   ├── models.rs    # Data models
│       │   ├── repositories/ # Data access
│       │   └── schema.rs    # Migrations
│       └── main.rs          # Entry point
│
└── docs/                    # Documentation
\`\`\`

## Development Commands

\`\`\`bash
# Frontend development
pnpm dev                    # Run Vite dev server
pnpm build                  # Build frontend
pnpm preview                # Preview production build
pnpm check                  # Type check Svelte
pnpm lint                   # Run ESLint
pnpm format                 # Format code with Prettier

# Tauri development
pnpm tauri dev              # Run app in development mode
pnpm tauri build            # Build production app
cargo test                  # Run Rust tests

# Backend only
cd src-tauri
cargo build                 # Build Rust backend
cargo run                   # Run backend (no GUI)
cargo test                  # Run tests
\`\`\`

## Database

The SQLite database is stored at:
- **Windows**: `C:\Users\{USERNAME}\AppData\Roaming\zweites-gehirn\db\zweites_gehirn.db`
- **macOS**: `~/Library/Application Support/zweites-gehirn/db/zweites_gehirn.db`
- **Linux**: `~/.local/share/zweites-gehirn/db/zweites_gehirn.db`

### Schema Migrations

Migrations run automatically on app startup. See `src-tauri/src/db/schema.rs` for migration code.

## Architecture

### Frontend (Svelte + SvelteKit)
- **Static Adapter**: Compiles to static files for Tauri
- **TypeScript**: Full type safety across the app
- **Stores**: Svelte stores for reactive state management
- **API Layer**: Thin wrapper around Tauri commands

### Backend (Rust + Tauri)
- **Tauri 2.0**: Modern desktop app framework
- **SQLite**: Fast local database with WAL mode
- **Rusqlite**: Direct database access with connection pooling
- **Service Layer**: Business logic separation
- **Repository Pattern**: Clean data access abstraction

### Communication
- **Tauri IPC**: Type-safe communication between frontend and backend
- **Commands**: Async Rust functions callable from JavaScript
- **Events**: Publish-subscribe for real-time updates (future)

## Troubleshooting

### App won't start
1. Check that Rust and Node.js are installed: `rustc --version && node --version`
2. Ensure dependencies are installed: `pnpm install`
3. Try cleaning and rebuilding:
   \`\`\`bash
   cd src-tauri && cargo clean && cd ..
   pnpm tauri build
   \`\`\`

### Database errors
1. Delete the database and restart (data will be lost):
   - Windows: Delete `%APPDATA%\zweites-gehirn\db\`
   - macOS/Linux: Delete `~/.local/share/zweites-gehirn/db/`

### Hot reload not working
1. Kill any running processes
2. Restart the dev server: `pnpm tauri dev`

## Performance Targets

- App launch: < 2 seconds
- Task creation: < 100ms
- Database queries: < 50ms
- Bundle size: < 15MB
- Memory usage: < 150MB

## Contributing

This is currently a personal project, but contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

[To be determined - currently unlicensed]

## Roadmap

See [PLAN.md](C:\Users\Operation1\.claude\plans\lively-frolicking-teacup.md) for the complete development roadmap.

### Current Phase: MVP (Phase 1)
- ✅ Quick Capture
- ✅ Task List
- ✅ Database Setup

### Next Phase: ADHD Features (Phase 2)
- Kanban Board
- Task Breakdown
- Pomodoro Timer
- Visual Enhancements

### Future Phases
- AI Brain Dump (Phase 3)
- Smart Prioritization (Phase 4)
- Gamification (Phase 5)
- Meeting Notes (Phase 6)
- Multi-user Organizations (Phase 8+)

## Support

For issues, questions, or feature requests, please open an issue on GitHub.

---

Built with ❤️ for people with ADHD who need a better way to manage their thoughts and tasks.
