use crate::db::models::*;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn create_task(
    state: State<AppState>,
    input: CreateTaskInput,
) -> Result<Task, String> {
    state
        .task_service
        .create_task(input)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks(
    state: State<AppState>,
    status: Option<String>,
) -> Result<Vec<Task>, String> {
    state
        .task_service
        .get_all_tasks(status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_task_by_id(
    state: State<AppState>,
    id: i64,
) -> Result<Task, String> {
    state
        .task_service
        .get_task(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task(
    state: State<AppState>,
    id: i64,
    input: UpdateTaskInput,
) -> Result<Task, String> {
    state
        .task_service
        .update_task(id, input)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(
    state: State<AppState>,
    id: i64,
) -> Result<(), String> {
    state
        .task_service
        .delete_task(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn complete_task(
    state: State<AppState>,
    id: i64,
) -> Result<Task, String> {
    state
        .task_service
        .complete_task(id)
        .map_err(|e| e.to_string())
}

// Subtask commands
#[tauri::command]
pub fn get_task_with_subtasks(
    state: State<AppState>,
    id: i64,
) -> Result<TaskWithSubtasks, String> {
    state
        .task_service
        .get_task_with_subtasks(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_subtasks(
    state: State<AppState>,
    parent_id: i64,
) -> Result<Vec<Task>, String> {
    state
        .task_service
        .get_subtasks(parent_id)
        .map_err(|e| e.to_string())
}

// Temporary debug command for database verification
#[tauri::command]
pub fn debug_database(state: State<AppState>) -> Result<serde_json::Value, String> {
    use rusqlite::Error as RusqliteError;
    use serde_json::json;

    let pool = &state.db_pool;
    let conn = pool.get().map_err(|e: r2d2::Error| e.to_string())?;

    // Query all tables
    let tasks: Vec<Task> = conn
        .prepare("SELECT * FROM tasks ORDER BY id")
        .map_err(|e: RusqliteError| e.to_string())?
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                user_id: row.get(1)?,
                workspace_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                project_id: row.get(5)?,
                status: row.get(6)?,
                priority: row.get(7)?,
                estimated_minutes: row.get(8)?,
                difficulty_level: row.get(9)?,
                energy_level: row.get(10)?,
                scheduled_date: row.get(11)?,
                due_date: row.get(12)?,
                completed_at: row.get(13)?,
                parent_task_id: row.get(14)?,
                order_index: row.get(15)?,
                column_position: row.get(16)?,
                tags: row.get(17)?,
                created_at: row.get(18)?,
                updated_at: row.get(19)?,
            })
        })
        .map_err(|e: RusqliteError| e.to_string())?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e: RusqliteError| e.to_string())?;

    let users: Vec<serde_json::Value> = conn
        .prepare("SELECT id, display_name, email, created_at FROM users")
        .map_err(|e: RusqliteError| e.to_string())?
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "display_name": row.get::<_, String>(1)?,
                "email": row.get::<_, Option<String>>(2)?,
                "created_at": row.get::<_, String>(3)?
            }))
        })
        .map_err(|e: RusqliteError| e.to_string())?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e: RusqliteError| e.to_string())?;

    let workspaces: Vec<serde_json::Value> = conn
        .prepare("SELECT id, name, workspace_type, owner_user_id, created_at FROM workspaces")
        .map_err(|e: RusqliteError| e.to_string())?
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "workspace_type": row.get::<_, String>(2)?,
                "owner_user_id": row.get::<_, Option<i64>>(3)?,
                "created_at": row.get::<_, String>(4)?
            }))
        })
        .map_err(|e: RusqliteError| e.to_string())?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e: RusqliteError| e.to_string())?;

    let schema_versions: Vec<serde_json::Value> = conn
        .prepare("SELECT version, applied_at FROM schema_version")
        .map_err(|e: RusqliteError| e.to_string())?
        .query_map([], |row| {
            Ok(json!({
                "version": row.get::<_, i64>(0)?,
                "applied_at": row.get::<_, String>(1)?
            }))
        })
        .map_err(|e: RusqliteError| e.to_string())?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e: RusqliteError| e.to_string())?;

    Ok(json!({
        "tasks": tasks,
        "users": users,
        "workspaces": workspaces,
        "schema_versions": schema_versions
    }))
}
