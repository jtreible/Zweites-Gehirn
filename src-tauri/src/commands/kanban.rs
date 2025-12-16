use crate::db::models::*;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn move_task_to_column(
    state: State<AppState>,
    task_id: i64,
    new_status: String,
    position: i32,
) -> Result<Task, String> {
    let pool = &state.db_pool;
    let conn = pool.get().map_err(|e| e.to_string())?;

    // Update task status and position
    conn.execute(
        "UPDATE tasks SET status = ?1, column_position = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
        [&new_status, &position.to_string(), &task_id.to_string()],
    )
    .map_err(|e| e.to_string())?;

    // Get updated task
    state
        .task_service
        .get_task(task_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks_by_status(
    state: State<AppState>,
    status: String,
) -> Result<Vec<Task>, String> {
    state
        .task_service
        .get_all_tasks(Some(status))
        .map_err(|e| e.to_string())
}
