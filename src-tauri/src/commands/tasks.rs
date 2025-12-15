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
