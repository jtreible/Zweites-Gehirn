use crate::db::{models::*, DbPool};
use anyhow::Result;
use rusqlite::params;

pub struct TaskRepository {
    pool: DbPool,
}

impl TaskRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new task
    pub fn create(&self, input: CreateTaskInput) -> Result<Task> {
        let conn = self.pool.get()?;
        let tags_json = input.tags.map(|t| serde_json::to_string(&t).unwrap());

        conn.execute(
            "INSERT INTO tasks (
                title, description, project_id, estimated_minutes,
                difficulty_level, energy_level, scheduled_date, due_date,
                parent_task_id, tags
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                input.title,
                input.description,
                input.project_id,
                input.estimated_minutes,
                input.difficulty_level,
                input.energy_level,
                input.scheduled_date,
                input.due_date,
                input.parent_task_id,
                tags_json,
            ],
        )?;

        let id = conn.last_insert_rowid();
        self.get_by_id(id)
    }

    /// Get task by ID
    pub fn get_by_id(&self, id: i64) -> Result<Task> {
        let conn = self.pool.get()?;
        let task = conn.query_row(
            "SELECT id, user_id, workspace_id, title, description, project_id,
                    status, priority, estimated_minutes, difficulty_level,
                    energy_level, scheduled_date, due_date, completed_at,
                    parent_task_id, order_index, column_position, tags, created_at, updated_at
             FROM tasks WHERE id = ?1",
            [id],
            |row| {
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
            },
        )?;

        Ok(task)
    }

    /// Get all tasks with optional filters
    pub fn get_all(&self, status: Option<String>) -> Result<Vec<Task>> {
        let conn = self.pool.get()?;

        let query = match status {
            Some(_) => {
                "SELECT id, user_id, workspace_id, title, description, project_id,
                        status, priority, estimated_minutes, difficulty_level,
                        energy_level, scheduled_date, due_date, completed_at,
                        parent_task_id, order_index, column_position, tags, created_at, updated_at
                 FROM tasks WHERE status = ?1 ORDER BY column_position, order_index, created_at DESC"
            }
            None => {
                "SELECT id, user_id, workspace_id, title, description, project_id,
                        status, priority, estimated_minutes, difficulty_level,
                        energy_level, scheduled_date, due_date, completed_at,
                        parent_task_id, order_index, column_position, tags, created_at, updated_at
                 FROM tasks ORDER BY column_position, order_index, created_at DESC"
            }
        };

        let mut stmt = conn.prepare(query)?;

        let tasks = if let Some(status_val) = status {
            stmt.query_map([status_val], Self::map_task_row)?
        } else {
            stmt.query_map([], Self::map_task_row)?
        };

        let tasks: Result<Vec<Task>, _> = tasks.collect();
        Ok(tasks?)
    }

    /// Update a task
    pub fn update(&self, id: i64, input: UpdateTaskInput) -> Result<Task> {
        let conn = self.pool.get()?;

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(title) = &input.title {
            updates.push("title = ?");
            params.push(Box::new(title.clone()));
        }
        if let Some(description) = &input.description {
            updates.push("description = ?");
            params.push(Box::new(description.clone()));
        }
        if let Some(status) = &input.status {
            updates.push("status = ?");
            params.push(Box::new(status.clone()));
        }
        if let Some(priority) = input.priority {
            updates.push("priority = ?");
            params.push(Box::new(priority));
        }
        if let Some(scheduled_date) = &input.scheduled_date {
            updates.push("scheduled_date = ?");
            params.push(Box::new(scheduled_date.clone()));
        }
        if let Some(due_date) = &input.due_date {
            updates.push("due_date = ?");
            params.push(Box::new(due_date.clone()));
        }

        updates.push("updated_at = CURRENT_TIMESTAMP");

        if updates.is_empty() {
            return self.get_by_id(id);
        }

        let query = format!("UPDATE tasks SET {} WHERE id = ?", updates.join(", "));
        params.push(Box::new(id));

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();

        conn.execute(&query, params_refs.as_slice())?;

        self.get_by_id(id)
    }

    /// Delete a task
    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Complete a task
    pub fn complete(&self, id: i64) -> Result<Task> {
        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE tasks SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
            [id],
        )?;
        self.get_by_id(id)
    }

    /// Get subtasks for a parent task
    pub fn get_subtasks(&self, parent_id: i64) -> Result<Vec<Task>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, workspace_id, title, description, project_id,
                    status, priority, estimated_minutes, difficulty_level,
                    energy_level, scheduled_date, due_date, completed_at,
                    parent_task_id, order_index, column_position, tags, created_at, updated_at
             FROM tasks WHERE parent_task_id = ?1 ORDER BY order_index, created_at",
        )?;

        let tasks = stmt.query_map([parent_id], Self::map_task_row)?;
        let tasks: Result<Vec<Task>, _> = tasks.collect();
        Ok(tasks?)
    }

    /// Get task with all subtasks and progress
    pub fn get_with_subtasks(&self, id: i64) -> Result<TaskWithSubtasks> {
        let task = self.get_by_id(id)?;
        let subtasks = self.get_subtasks(id)?;

        let total = subtasks.len() as i32;
        let completed = subtasks
            .iter()
            .filter(|t| t.status == "completed")
            .count() as i32;
        let percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        let progress = SubtaskProgress {
            total,
            completed,
            percentage,
        };

        Ok(TaskWithSubtasks {
            task,
            subtasks,
            progress,
        })
    }

    /// Helper to map row to Task
    fn map_task_row(row: &rusqlite::Row) -> rusqlite::Result<Task> {
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
    }
}
