use crate::db::{models::*, repositories::TaskRepository, DbPool};
use anyhow::Result;

pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repository: TaskRepository::new(pool),
        }
    }

    pub fn create_task(&self, input: CreateTaskInput) -> Result<Task> {
        self.repository.create(input)
    }

    pub fn get_task(&self, id: i64) -> Result<Task> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_tasks(&self, status: Option<String>) -> Result<Vec<Task>> {
        self.repository.get_all(status)
    }

    pub fn update_task(&self, id: i64, input: UpdateTaskInput) -> Result<Task> {
        self.repository.update(id, input)
    }

    pub fn delete_task(&self, id: i64) -> Result<()> {
        self.repository.delete(id)
    }

    pub fn complete_task(&self, id: i64) -> Result<Task> {
        self.repository.complete(id)
    }

    // Subtask methods
    pub fn get_task_with_subtasks(&self, id: i64) -> Result<TaskWithSubtasks> {
        self.repository.get_with_subtasks(id)
    }

    pub fn get_subtasks(&self, parent_id: i64) -> Result<Vec<Task>> {
        self.repository.get_subtasks(parent_id)
    }
}
