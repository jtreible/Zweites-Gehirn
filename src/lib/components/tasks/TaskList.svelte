<script lang="ts">
	import { todoTasks, completedTasks, loading, error } from '$stores/tasks';
	import TaskCard from './TaskCard.svelte';

	let filter = 'todo'; // 'todo' | 'completed' | 'all'

	$: displayTasks = filter === 'todo' ? $todoTasks : filter === 'completed' ? $completedTasks : [...$todoTasks, ...$completedTasks];
</script>

<div class="task-list-container">
	<div class="header">
		<h2>Your Tasks</h2>
		<div class="filter-tabs">
			<button class:active={filter === 'todo'} on:click={() => filter = 'todo'}>
				To Do ({$todoTasks.length})
			</button>
			<button class:active={filter === 'completed'} on:click={() => filter = 'completed'}>
				Completed ({$completedTasks.length})
			</button>
		</div>
	</div>

	{#if $loading}
		<div class="loading">Loading tasks...</div>
	{:else if $error}
		<div class="error">{$error}</div>
	{:else if displayTasks.length === 0}
		<div class="empty-state">
			{#if filter === 'todo'}
				<p>No tasks yet! Add one above to get started.</p>
			{:else}
				<p>No completed tasks yet.</p>
			{/if}
		</div>
	{:else}
		<div class="task-list">
			{#each displayTasks as task (task.id)}
				<TaskCard {task} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.task-list-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.filter-tabs {
		display: flex;
		gap: 0.5rem;
	}

	.filter-tabs button {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		border: 1px solid rgba(255, 255, 255, 0.2);
		background: transparent;
		color: inherit;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.9rem;
	}

	.filter-tabs button:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.filter-tabs button.active {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-color: transparent;
	}

	.task-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.loading,
	.error,
	.empty-state {
		padding: 2rem;
		text-align: center;
		opacity: 0.7;
		font-size: 1.1rem;
	}

	.error {
		color: #ff6b6b;
	}
</style>
