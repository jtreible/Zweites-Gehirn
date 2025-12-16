use anyhow::Result;
use rusqlite::Connection;

/// Run all database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Set journal mode (this PRAGMA returns results, so we use pragma_update)
    conn.pragma_update(None, "journal_mode", "WAL")?;

    // Version tracking
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    let current_version = get_current_version(conn)?;

    // Apply migrations in order
    if current_version < 1 {
        migration_v001(conn)?;
    }
    if current_version < 2 {
        migration_v002(conn)?;
    }

    Ok(())
}

fn get_current_version(conn: &Connection) -> Result<i32> {
    let version: Result<i32, _> = conn.query_row(
        "SELECT MAX(version) FROM schema_version",
        [],
        |row| row.get(0),
    );

    Ok(version.unwrap_or(0))
}

fn set_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [version],
    )?;
    Ok(())
}

/// Migration V001: Initial schema
fn migration_v001(conn: &Connection) -> Result<()> {
    tracing::info!("Running migration v001: Initial schema");

    // Users table (for multi-user support, single default user for now)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE,
            display_name TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Workspaces (single personal workspace for Phase 1)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workspaces (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            workspace_type TEXT NOT NULL DEFAULT 'personal',
            owner_user_id INTEGER,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (owner_user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Projects
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            icon TEXT,
            archived INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Tasks
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 1,
            workspace_id INTEGER NOT NULL DEFAULT 1,
            title TEXT NOT NULL,
            description TEXT,
            project_id INTEGER,
            status TEXT NOT NULL DEFAULT 'todo',
            priority INTEGER DEFAULT 0,
            estimated_minutes INTEGER,
            difficulty_level INTEGER,
            energy_level TEXT,
            scheduled_date TEXT,
            due_date TEXT,
            completed_at TEXT,
            parent_task_id INTEGER,
            order_index INTEGER DEFAULT 0,
            tags TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
            FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Indexes for tasks
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_scheduled_date ON tasks(scheduled_date)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id)", [])?;

    // Full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS tasks_fts USING fts5(
            title,
            description,
            content=tasks,
            content_rowid=id
        )",
        [],
    )?;

    // FTS triggers
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tasks_fts_insert AFTER INSERT ON tasks BEGIN
            INSERT INTO tasks_fts(rowid, title, description)
            VALUES (new.id, new.title, new.description);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tasks_fts_update AFTER UPDATE ON tasks BEGIN
            UPDATE tasks_fts
            SET title = new.title, description = new.description
            WHERE rowid = new.id;
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tasks_fts_delete AFTER DELETE ON tasks BEGIN
            DELETE FROM tasks_fts WHERE rowid = old.id;
        END",
        [],
    )?;

    // User preferences
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_preferences (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL DEFAULT 1,
            theme TEXT DEFAULT 'system',
            default_view TEXT DEFAULT 'list',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create default user and workspace for single-user mode
    conn.execute(
        "INSERT OR IGNORE INTO users (id, display_name) VALUES (1, 'Default User')",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO workspaces (id, name, workspace_type, owner_user_id)
         VALUES (1, 'Personal', 'personal', 1)",
        [],
    )?;

    set_version(conn, 1)?;
    tracing::info!("Migration v001 completed");
    Ok(())
}

/// Migration v002: Add Kanban support
fn migration_v002(conn: &Connection) -> Result<()> {
    tracing::info!("Running migration v002: Kanban support");

    // Add column_position for Kanban sorting
    conn.execute(
        "ALTER TABLE tasks ADD COLUMN column_position INTEGER DEFAULT 0",
        [],
    )?;

    // Add index for parent_task_id (for subtasks performance)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_task_id)",
        [],
    )?;

    set_version(conn, 2)?;
    tracing::info!("Migration v002 completed");
    Ok(())
}
