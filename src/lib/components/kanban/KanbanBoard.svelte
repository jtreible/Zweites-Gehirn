<script lang="ts">
	import { onMount } from 'svelte';
	import type { Task } from '$types/task';
	import { getTasks, moveTaskToColumn } from '$lib/api/tasks';
	import KanbanColumn from './KanbanColumn.svelte';

	let todoTasks: Task[] = [];
	let inProgressTasks: Task[] = [];
	let completedTasks: Task[] = [];
	let isLoading = true;

	const columns = [
		{ id: 'todo', title: 'To Do', color: '#667eea' },
		{ id: 'in_progress', title: 'In Progress', color: '#f59e0b' },
		{ id: 'completed', title: 'Completed', color: '#10b981' }
	];

	onMount(async () => {
		await loadTasks();
	});

	async function loadTasks() {
		isLoading = true;
		try {
			const allTasks = await getTasks();
			// Filter out subtasks (tasks with parent_task_id)
			const mainTasks = allTasks.filter((t) => !t.parent_task_id);
			todoTasks = mainTasks.filter((t) => t.status === 'todo');
			inProgressTasks = mainTasks.filter((t) => t.status === 'in_progress');
			completedTasks = mainTasks.filter((t) => t.status === 'completed');
		} catch (error) {
			console.error('Error loading tasks:', error);
		} finally {
			isLoading = false;
		}
	}

	async function handleMoveTask(taskId: number, newStatus: string) {
		try {
			await moveTaskToColumn(taskId, newStatus, 0);
			await loadTasks();
		} catch (error) {
			console.error('Error moving task:', error);
		}
	}

	$: tasksByStatus = {
		todo: todoTasks,
		in_progress: inProgressTasks,
		completed: completedTasks
	};
</script>

<div class="kanban-board">
	{#if isLoading}
		<p class="loading">Loading tasks...</p>
	{:else}
		{#each columns as column}
			<KanbanColumn
				title={column.title}
				tasks={tasksByStatus[column.id]}
				status={column.id}
				color={column.color}
				onMoveTask={handleMoveTask}
			/>
		{/each}
	{/if}
</div>

<style>
	.kanban-board {
		display: flex;
		gap: 1.5rem;
		padding: 2rem;
		overflow-x: auto;
		min-height: 600px;
	}

	.loading {
		text-align: center;
		font-size: 1.2rem;
		color: #666;
		margin: 2rem;
	}
</style>
