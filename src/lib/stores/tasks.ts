import { writable, derived } from 'svelte/store';
import type { Task, CreateTaskInput, UpdateTaskInput } from '$types/task';
import * as taskApi from '$lib/api/tasks';

// Store for all tasks
export const tasks = writable<Task[]>([]);

// Loading state
export const loading = writable(false);

// Error state
export const error = writable<string | null>(null);

// Derived stores for filtered tasks (exclude subtasks - they have parent_task_id)
export const activeTasks = derived(tasks, ($tasks) =>
	$tasks.filter((t) => (t.status === 'todo' || t.status === 'in_progress') && !t.parent_task_id).sort((a, b) => a.order_index - b.order_index)
);

export const todoTasks = derived(tasks, ($tasks) =>
	$tasks.filter((t) => t.status === 'todo' && !t.parent_task_id).sort((a, b) => a.order_index - b.order_index)
);

export const inProgressTasks = derived(tasks, ($tasks) =>
	$tasks.filter((t) => t.status === 'in_progress' && !t.parent_task_id).sort((a, b) => a.order_index - b.order_index)
);

export const completedTasks = derived(tasks, ($tasks) =>
	$tasks.filter((t) => t.status === 'completed' && !t.parent_task_id)
);

export const todayTasks = derived(tasks, ($tasks) => {
	const today = new Date().toISOString().split('T')[0];
	return $tasks.filter(
		(t) => t.scheduled_date === today || t.due_date === today
	);
});

// Actions
export async function loadTasks(status?: string) {
	loading.set(true);
	error.set(null);
	try {
		const fetchedTasks = await taskApi.getTasks(status);
		tasks.set(fetchedTasks);
	} catch (e) {
		error.set(e instanceof Error ? e.message : 'Failed to load tasks');
		console.error('Failed to load tasks:', e);
	} finally {
		loading.set(false);
	}
}

export async function addTask(input: CreateTaskInput) {
	loading.set(true);
	error.set(null);
	try {
		const newTask = await taskApi.createTask(input);
		tasks.update((current) => [newTask, ...current]);
		return newTask;
	} catch (e) {
		error.set(e instanceof Error ? e.message : 'Failed to create task');
		console.error('Failed to create task:', e);
		throw e;
	} finally {
		loading.set(false);
	}
}

export async function modifyTask(id: number, input: UpdateTaskInput) {
	loading.set(true);
	error.set(null);
	try {
		const updatedTask = await taskApi.updateTask(id, input);
		tasks.update((current) => current.map((t) => (t.id === id ? updatedTask : t)));
		return updatedTask;
	} catch (e) {
		error.set(e instanceof Error ? e.message : 'Failed to update task');
		console.error('Failed to update task:', e);
		throw e;
	} finally {
		loading.set(false);
	}
}

export async function removeTask(id: number) {
	loading.set(true);
	error.set(null);
	try {
		await taskApi.deleteTask(id);
		tasks.update((current) => current.filter((t) => t.id !== id));
	} catch (e) {
		error.set(e instanceof Error ? e.message : 'Failed to delete task');
		console.error('Failed to delete task:', e);
		throw e;
	} finally {
		loading.set(false);
	}
}

export async function markTaskComplete(id: number) {
	loading.set(true);
	error.set(null);
	try {
		const completedTask = await taskApi.completeTask(id);
		tasks.update((current) => current.map((t) => (t.id === id ? completedTask : t)));
		return completedTask;
	} catch (e) {
		error.set(e instanceof Error ? e.message : 'Failed to complete task');
		console.error('Failed to complete task:', e);
		throw e;
	} finally {
		loading.set(false);
	}
}
