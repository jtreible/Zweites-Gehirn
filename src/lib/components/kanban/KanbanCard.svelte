<script lang="ts">
	import type { Task } from '$types/task';
	import { getSubtasks } from '$lib/api/tasks';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
	import SubtaskList from '$lib/components/tasks/SubtaskList.svelte';

	export let task: Task;
	export let onMoveTask: (taskId: number, newStatus: string) => Promise<void>;
	export let currentStatus: string;

	const statusOptions = [
		{ id: 'todo', label: 'To Do' },
		{ id: 'in_progress', label: 'In Progress' },
		{ id: 'completed', label: 'Completed' }
	];

	let showMoveMenu = false;
	let isExpanded = false;
	let subtasks: Task[] = [];
	let isLoadingSubtasks = false;

	function toggleMoveMenu() {
		showMoveMenu = !showMoveMenu;
	}

	async function moveTask(newStatus: string) {
		showMoveMenu = false;
		await onMoveTask(task.id, newStatus);
	}

	async function toggleExpand() {
		isExpanded = !isExpanded;
		if (isExpanded && subtasks.length === 0 && !isLoadingSubtasks) {
			await loadSubtasks();
		}
	}

	async function loadSubtasks() {
		isLoadingSubtasks = true;
		try {
			subtasks = await getSubtasks(task.id);
		} catch (error) {
			console.error('Error loading subtasks:', error);
		} finally {
			isLoadingSubtasks = false;
		}
	}

	$: progress = {
		total: subtasks.length,
		completed: subtasks.filter((t) => t.status === 'completed').length,
		percentage:
			subtasks.length > 0 ? (subtasks.filter((t) => t.status === 'completed').length / subtasks.length) * 100 : 0
	};

	$: hasSubtasks = subtasks.length > 0;
</script>

<div class="kanban-card">
	<div class="card-header">
		<h4>{task.title}</h4>
		<div class="header-actions">
			<button class="expand-btn" on:click={toggleExpand} title="Toggle subtasks">
				{isExpanded ? '▼' : '▶'}
			</button>
			<button class="move-btn" on:click={toggleMoveMenu}>⋮</button>
		</div>
	</div>
	{#if task.description}
		<p class="description">{task.description}</p>
	{/if}
	<div class="meta">
		{#if task.difficulty_level}
			<span class="difficulty">{'🌶️'.repeat(task.difficulty_level)}</span>
		{/if}
		{#if task.estimated_minutes}
			<span class="time">⏱️ {task.estimated_minutes}min</span>
		{/if}
	</div>

	{#if hasSubtasks}
		<ProgressBar {progress} />
	{/if}

	{#if isExpanded}
		<div class="subtasks-section">
			{#if isLoadingSubtasks}
				<p class="loading">Loading subtasks...</p>
			{:else}
				<SubtaskList {subtasks} parentId={task.id} />
			{/if}
		</div>
	{/if}

	{#if showMoveMenu}
		<div class="move-menu">
			{#each statusOptions as option}
				{#if option.id !== currentStatus}
					<button on:click={() => moveTask(option.id)}>Move to {option.label}</button>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.kanban-card {
		background: white;
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 0.75rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
		position: relative;
	}

	.kanban-card:hover {
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 0.5rem;
	}

	h4 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #333;
		flex: 1;
	}

	.header-actions {
		display: flex;
		gap: 0.25rem;
		align-items: center;
	}

	.expand-btn {
		background: transparent;
		border: none;
		font-size: 0.8rem;
		color: #888;
		cursor: pointer;
		padding: 0.25rem;
		line-height: 1;
		transition: all 0.2s;
	}

	.expand-btn:hover {
		color: #333;
		transform: scale(1.1);
	}

	.move-btn {
		background: transparent;
		border: none;
		font-size: 1.2rem;
		color: #888;
		cursor: pointer;
		padding: 0 0.25rem;
		line-height: 1;
	}

	.move-btn:hover {
		color: #333;
	}

	.description {
		font-size: 0.85rem;
		color: #666;
		margin: 0 0 0.5rem 0;
	}

	.meta {
		display: flex;
		gap: 0.75rem;
		font-size: 0.85rem;
		color: #888;
	}

	.difficulty,
	.time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.move-menu {
		position: absolute;
		top: 2.5rem;
		right: 0.5rem;
		background: white;
		border: 1px solid #ddd;
		border-radius: 6px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		padding: 0.5rem;
		z-index: 10;
		min-width: 150px;
	}

	.move-menu button {
		display: block;
		width: 100%;
		padding: 0.5rem;
		background: transparent;
		border: none;
		text-align: left;
		cursor: pointer;
		font-size: 0.9rem;
		color: #333;
		border-radius: 4px;
	}

	.move-menu button:hover {
		background: #f0f0f0;
	}

	.subtasks-section {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid #e0e0e0;
	}

	.loading {
		font-size: 0.85rem;
		color: #888;
		font-style: italic;
		text-align: center;
		padding: 0.5rem;
	}
</style>
