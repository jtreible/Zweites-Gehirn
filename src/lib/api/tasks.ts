import { invoke } from '@tauri-apps/api/core';
import type { Task, CreateTaskInput, UpdateTaskInput, TaskWithSubtasks } from '$types/task';

export async function createTask(input: CreateTaskInput): Promise<Task> {
	return invoke('create_task', { input });
}

export async function getTasks(status?: string): Promise<Task[]> {
	return invoke('get_tasks', { status });
}

export async function getTaskById(id: number): Promise<Task> {
	return invoke('get_task_by_id', { id });
}

export async function updateTask(id: number, input: UpdateTaskInput): Promise<Task> {
	return invoke('update_task', { id, input });
}

export async function deleteTask(id: number): Promise<void> {
	return invoke('delete_task', { id });
}

export async function completeTask(id: number): Promise<Task> {
	return invoke('complete_task', { id });
}

// Subtask APIs
export async function getTaskWithSubtasks(id: number): Promise<TaskWithSubtasks> {
	return invoke('get_task_with_subtasks', { id });
}

export async function getSubtasks(parent_id: number): Promise<Task[]> {
	return invoke('get_subtasks', { parentId: parent_id });
}

// Kanban APIs
export async function moveTaskToColumn(
	taskId: number,
	newStatus: string,
	position: number
): Promise<Task> {
	return invoke('move_task_to_column', { taskId, newStatus, position });
}

export async function getTasksByStatus(status: string): Promise<Task[]> {
	return invoke('get_tasks_by_status', { status });
}
