<script lang="ts">
	import type { Task } from '$types/task';
	import KanbanCard from './KanbanCard.svelte';

	export let title: string;
	export let tasks: Task[];
	export let status: string;
	export let color: string = '#667eea';
	export let onMoveTask: (taskId: number, newStatus: string) => Promise<void>;
</script>

<div class="column">
	<div class="column-header" style="background: {color};">
		<h3>{title}</h3>
		<span class="count">{tasks.length}</span>
	</div>
	<div class="column-content">
		{#each tasks as task (task.id)}
			<KanbanCard {task} {onMoveTask} currentStatus={status} />
		{/each}
		{#if tasks.length === 0}
			<p class="empty-message">No tasks</p>
		{/if}
	</div>
</div>

<style>
	.column {
		background: #f5f5f5;
		border-radius: 12px;
		display: flex;
		flex-direction: column;
		min-height: 500px;
		width: 320px;
		flex-shrink: 0;
	}

	.column-header {
		padding: 1rem;
		border-radius: 12px 12px 0 0;
		display: flex;
		justify-content: space-between;
		align-items: center;
		color: white;
	}

	h3 {
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
	}

	.count {
		background: rgba(255, 255, 255, 0.3);
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
	}

	.column-content {
		padding: 1rem;
		flex: 1;
		overflow-y: auto;
	}

	.empty-message {
		text-align: center;
		color: #999;
		font-style: italic;
		margin-top: 2rem;
	}
</style>
