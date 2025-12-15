import { invoke } from '@tauri-apps/api/core';
import type { Task, CreateTaskInput, UpdateTaskInput } from '$types/task';

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
