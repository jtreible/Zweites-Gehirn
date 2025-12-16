pub mod models;
pub mod repositories;
pub mod schema;

use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::PathBuf;

pub type DbPool = Pool<SqliteConnectionManager>;

/// Initialize the database and return a connection pool
pub fn init_database() -> Result<DbPool> {
    let db_path = get_database_path()?;

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::new(manager)?;

    // Run migrations
    let conn = pool.get()?;
    schema::run_migrations(&conn)?;

    tracing::info!("Database initialized at: {}", db_path.display());
    Ok(pool)
}

/// Get the path to the database file
fn get_database_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;

    path.push("zweites-gehirn");
    path.push("db");
    path.push("zweites_gehirn.db");

    Ok(path)
}

#[cfg(test)]
pub fn init_test_database() -> Result<DbPool> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager)?;
    let conn = pool.get()?;
    schema::run_migrations(&conn)?;
    Ok(pool)
}
