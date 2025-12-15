use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub workspace_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub status: String,
    pub priority: i32,
    pub estimated_minutes: Option<i32>,
    pub difficulty_level: Option<i32>,
    pub energy_level: Option<String>,
    pub scheduled_date: Option<String>,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub parent_task_id: Option<i64>,
    pub order_index: i32,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub estimated_minutes: Option<i32>,
    pub difficulty_level: Option<i32>,
    pub energy_level: Option<String>,
    pub scheduled_date: Option<String>,
    pub due_date: Option<String>,
    pub parent_task_id: Option<i64>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<i64>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub estimated_minutes: Option<i32>,
    pub difficulty_level: Option<i32>,
    pub energy_level: Option<String>,
    pub scheduled_date: Option<String>,
    pub due_date: Option<String>,
    pub parent_task_id: Option<i64>,
    pub order_index: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
}
