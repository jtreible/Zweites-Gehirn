<script lang="ts">
	import type { Task } from '$types/task';
	import { completeTask, createTask } from '$lib/api/tasks';
	import { loadTasks } from '$stores/tasks';

	export let subtasks: Task[];
	export let parentId: number;

	let newSubtaskTitle = '';
	let isAdding = false;

	async function toggleSubtask(subtask: Task) {
		try {
			if (subtask.status === 'completed') {
				// Uncomplete - update status back to todo
				await completeTask(subtask.id);
			} else {
				// Complete
				await completeTask(subtask.id);
			}
			await loadTasks();
		} catch (error) {
			console.error('Error toggling subtask:', error);
		}
	}

	async function addSubtask() {
		if (!newSubtaskTitle.trim()) return;

		try {
			await createTask({
				title: newSubtaskTitle.trim(),
				parent_task_id: parentId
			});
			newSubtaskTitle = '';
			isAdding = false;
			await loadTasks();
		} catch (error) {
			console.error('Error creating subtask:', error);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			addSubtask();
		} else if (event.key === 'Escape') {
			isAdding = false;
			newSubtaskTitle = '';
		}
	}
</script>

<div class="subtask-list">
	{#each subtasks as subtask}
		<div class="subtask-item">
			<input
				type="checkbox"
				checked={subtask.status === 'completed'}
				on:change={() => toggleSubtask(subtask)}
				id="subtask-{subtask.id}"
			/>
			<label for="subtask-{subtask.id}" class:completed={subtask.status === 'completed'}>
				{subtask.title}
			</label>
		</div>
	{/each}

	{#if isAdding}
		<div class="add-subtask">
			<input
				type="text"
				bind:value={newSubtaskTitle}
				on:keydown={handleKeydown}
				placeholder="New subtask..."
				autofocus
			/>
			<button on:click={addSubtask} class="btn-add">Add</button>
			<button on:click={() => (isAdding = false)} class="btn-cancel">Cancel</button>
		</div>
	{:else}
		<button class="btn-new-subtask" on:click={() => (isAdding = true)}>+ Add subtask</button>
	{/if}
</div>

<style>
	.subtask-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.5rem 0;
	}

	.subtask-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background: #f9f9f9;
		border-radius: 6px;
		transition: background 0.2s;
	}

	.subtask-item:hover {
		background: #f0f0f0;
	}

	.subtask-item input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.subtask-item label {
		flex: 1;
		cursor: pointer;
		user-select: none;
		transition: all 0.2s;
		color: #333;
	}

	.subtask-item label.completed {
		text-decoration: line-through;
		opacity: 0.6;
	}

	.add-subtask {
		display: flex;
		gap: 0.5rem;
		padding: 0.5rem;
	}

	.add-subtask input {
		flex: 1;
		padding: 0.5rem;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 0.9rem;
		color: #333;
		background: white;
	}

	.btn-add,
	.btn-cancel {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.9rem;
		transition: all 0.2s;
	}

	.btn-add {
		background: #667eea;
		color: white;
	}

	.btn-add:hover {
		background: #5568d3;
	}

	.btn-cancel {
		background: #e0e0e0;
		color: #666;
	}

	.btn-cancel:hover {
		background: #d0d0d0;
	}

	.btn-new-subtask {
		padding: 0.5rem;
		background: transparent;
		border: 1px dashed #ccc;
		border-radius: 6px;
		color: #667eea;
		cursor: pointer;
		font-size: 0.9rem;
		transition: all 0.2s;
	}

	.btn-new-subtask:hover {
		background: #f9f9f9;
		border-color: #667eea;
	}
</style>
