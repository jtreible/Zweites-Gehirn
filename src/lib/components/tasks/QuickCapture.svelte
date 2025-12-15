<script lang="ts">
	import { addTask } from '$stores/tasks';

	let title = '';
	let isAdding = false;

	async function handleSubmit() {
		if (!title.trim()) return;

		isAdding = true;
		try {
			await addTask({ title: title.trim() });
			title = '';
		} catch (e) {
			console.error('Failed to add task:', e);
		} finally {
			isAdding = false;
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSubmit();
		}
	}
</script>

<div class="quick-capture">
	<input
		type="text"
		bind:value={title}
		on:keydown={handleKeydown}
		placeholder="Quick add a task... (Press Enter)"
		disabled={isAdding}
		class="capture-input"
		autofocus
	/>
	<button on:click={handleSubmit} disabled={isAdding || !title.trim()} class="add-btn">
		{isAdding ? '...' : '+ Add'}
	</button>
</div>

<style>
	.quick-capture {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 12px;
		border: 2px solid rgba(255, 255, 255, 0.1);
		transition: border-color 0.2s;
	}

	.quick-capture:focus-within {
		border-color: #667eea;
	}

	.capture-input {
		flex: 1;
		padding: 0.75rem 1rem;
		font-size: 1rem;
		border: none;
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		color: inherit;
		outline: none;
		transition: background 0.2s;
	}

	.capture-input:focus {
		background: rgba(255, 255, 255, 0.15);
	}

	.capture-input::placeholder {
		opacity: 0.5;
	}

	.add-btn {
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border: none;
		color: white;
		transition: transform 0.2s, opacity 0.2s;
	}

	.add-btn:hover:not(:disabled) {
		transform: scale(1.05);
	}

	.add-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
