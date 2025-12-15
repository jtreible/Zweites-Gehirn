use crate::db::DbPool;
use crate::services::TaskService;
use std::sync::Arc;

pub struct AppState {
    pub task_service: Arc<TaskService>,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        Self {
            task_service: Arc::new(TaskService::new(pool)),
        }
    }
}
