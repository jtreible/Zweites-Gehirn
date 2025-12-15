// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod services;
mod state;

use state::AppState;
use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize database
    let db_pool = db::init_database().expect("Failed to initialize database");

    // Create application state
    let app_state = AppState::new(db_pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::tasks::create_task,
            commands::tasks::get_tasks,
            commands::tasks::get_task_by_id,
            commands::tasks::update_task,
            commands::tasks::delete_task,
            commands::tasks::complete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
