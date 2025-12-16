<script lang="ts">
	import type { Task } from '$types/task';
	import { markTaskComplete, removeTask } from '$stores/tasks';
	import { getSubtasks } from '$lib/api/tasks';
	import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
	import SubtaskList from './SubtaskList.svelte';
	import DifficultyIndicator from '$lib/components/ui/DifficultyIndicator.svelte';
	import EnergyIndicator from '$lib/components/ui/EnergyIndicator.svelte';
	import PriorityIndicator from '$lib/components/ui/PriorityIndicator.svelte';

	export let task: Task;

	let isProcessing = false;
	let isExpanded = false;
	let subtasks: Task[] = [];
	let isLoadingSubtasks = false;

	async function handleComplete() {
		if (isProcessing) return;
		isProcessing = true;
		try {
			await markTaskComplete(task.id);
		} catch (e) {
			console.error('Failed to complete task:', e);
		} finally {
			isProcessing = false;
		}
	}

	async function handleDelete() {
		if (isProcessing) return;
		if (!confirm('Are you sure you want to delete this task?')) return;

		isProcessing = true;
		try {
			await removeTask(task.id);
		} catch (e) {
			console.error('Failed to delete task:', e);
		} finally {
			isProcessing = false;
		}
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

	function formatDate(dateString?: string): string {
		if (!dateString) return '';
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	$: progress = {
		total: subtasks.length,
		completed: subtasks.filter((t) => t.status === 'completed').length,
		percentage:
			subtasks.length > 0 ? (subtasks.filter((t) => t.status === 'completed').length / subtasks.length) * 100 : 0
	};

	$: hasSubtasks = subtasks.length > 0;
</script>

<div class="task-card" class:completed={task.status === 'completed'}>
	<div class="task-content">
		<div class="task-header">
			<h3 class:strike={task.status === 'completed'}>{task.title}</h3>
			<div class="header-indicators">
				<PriorityIndicator priority={task.priority} />
				<DifficultyIndicator level={task.difficulty_level} />
				<button class="expand-btn" on:click={toggleExpand} title="Toggle subtasks">
					{isExpanded ? '▼' : '▶'}
				</button>
			</div>
		</div>

		{#if task.description}
			<p class="description">{task.description}</p>
		{/if}

		<div class="task-meta">
			<EnergyIndicator level={task.energy_level} />
			{#if task.estimated_minutes}
				<span class="meta-item">⏱️ {task.estimated_minutes}min</span>
			{/if}
			{#if task.due_date}
				<span class="meta-item">📅 {formatDate(task.due_date)}</span>
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
	</div>

	<div class="task-actions">
		{#if task.status !== 'completed'}
			<button on:click={handleComplete} disabled={isProcessing} class="complete-btn" title="Mark as complete">
				✓
			</button>
		{/if}
		<button on:click={handleDelete} disabled={isProcessing} class="delete-btn" title="Delete task">
			🗑️
		</button>
	</div>
</div>

<style>
	.task-card {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 1rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 10px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		transition: all 0.2s;
	}

	.task-card:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.task-card.completed {
		opacity: 0.6;
	}

	.task-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.task-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.header-indicators {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-left: auto;
	}

	.expand-btn {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.8rem;
		padding: 0.25rem 0.5rem;
		opacity: 0.6;
		transition: all 0.2s;
	}

	.expand-btn:hover {
		opacity: 1;
		transform: scale(1.1);
	}

	h3 {
		font-size: 1.1rem;
		font-weight: 500;
		margin: 0;
		flex: 1;
	}

	h3.strike {
		text-decoration: line-through;
	}

	.difficulty {
		font-size: 0.9rem;
	}

	.description {
		font-size: 0.95rem;
		opacity: 0.8;
		margin: 0;
	}

	.task-meta {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.meta-item {
		font-size: 0.85rem;
		opacity: 0.7;
	}

	.task-actions {
		display: flex;
		gap: 0.5rem;
	}

	.task-actions button {
		padding: 0.5rem;
		width: 36px;
		height: 36px;
		border-radius: 6px;
		border: 1px solid rgba(255, 255, 255, 0.2);
		background: rgba(255, 255, 255, 0.05);
		cursor: pointer;
		transition: all 0.2s;
		font-size: 1rem;
	}

	.task-actions button:hover:not(:disabled) {
		transform: scale(1.1);
		background: rgba(255, 255, 255, 0.1);
	}

	.complete-btn:hover:not(:disabled) {
		background: rgba(76, 175, 80, 0.2);
		border-color: rgba(76, 175, 80, 0.5);
	}

	.delete-btn:hover:not(:disabled) {
		background: rgba(244, 67, 54, 0.2);
		border-color: rgba(244, 67, 54, 0.5);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.subtasks-section {
		margin-top: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.loading {
		font-size: 0.9rem;
		opacity: 0.6;
		font-style: italic;
	}
</style>
