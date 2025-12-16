<script lang="ts">
	import { addTask } from '$stores/tasks';

	let title = '';
	let priority = '';
	let difficulty_level: number | null = null;
	let energy_level = '';
	let isAdding = false;
	let showDetails = false;

	async function handleSubmit() {
		if (!title.trim()) return;

		isAdding = true;
		try {
			await addTask({
				title: title.trim(),
				priority: priority || undefined,
				difficulty_level: difficulty_level || undefined,
				energy_level: energy_level || undefined
			});
			// Reset form
			title = '';
			priority = '';
			difficulty_level = null;
			energy_level = '';
			showDetails = false;
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
	<div class="capture-row">
		<input
			type="text"
			bind:value={title}
			on:keydown={handleKeydown}
			placeholder="Quick add a task... (Press Enter)"
			disabled={isAdding}
			class="capture-input"
			autofocus
		/>
		<button
			type="button"
			on:click={() => (showDetails = !showDetails)}
			class="details-btn"
			title="Add details"
		>
			{showDetails ? '▲' : '▼'}
		</button>
		<button on:click={handleSubmit} disabled={isAdding || !title.trim()} class="add-btn">
			{isAdding ? '...' : '+ Add'}
		</button>
	</div>

	{#if showDetails}
		<div class="details-section">
			<div class="detail-group">
				<label for="priority">Priority</label>
				<select id="priority" bind:value={priority}>
					<option value="">None</option>
					<option value="low">🟢 Low</option>
					<option value="medium">🟡 Medium</option>
					<option value="high">🟠 High</option>
					<option value="urgent">🔴 Urgent</option>
				</select>
			</div>

			<div class="detail-group">
				<label for="difficulty">Difficulty</label>
				<select
					id="difficulty"
					bind:value={difficulty_level}
					on:change={(e) => (difficulty_level = e.currentTarget.value ? Number(e.currentTarget.value) : null)}
				>
					<option value="">None</option>
					<option value="1">🌶️ Easy</option>
					<option value="2">🌶️🌶️ Medium</option>
					<option value="3">🌶️🌶️🌶️ Hard</option>
					<option value="4">🌶️🌶️🌶️🌶️ Very Hard</option>
					<option value="5">🌶️🌶️🌶️🌶️🌶️ Extreme</option>
				</select>
			</div>

			<div class="detail-group">
				<label for="energy">Energy Level</label>
				<select id="energy" bind:value={energy_level}>
					<option value="">None</option>
					<option value="low">🔋 Low</option>
					<option value="medium">🔋🔋 Medium</option>
					<option value="high">🔋🔋🔋 High</option>
				</select>
			</div>
		</div>
	{/if}
</div>

<style>
	.quick-capture {
		display: flex;
		flex-direction: column;
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

	.capture-row {
		display: flex;
		gap: 0.5rem;
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

	.details-btn {
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 8px;
		color: inherit;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.9rem;
	}

	.details-btn:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.add-btn {
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border: none;
		color: white;
		border-radius: 8px;
		cursor: pointer;
		transition: transform 0.2s, opacity 0.2s;
	}

	.add-btn:hover:not(:disabled) {
		transform: scale(1.05);
	}

	.add-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.details-section {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		padding-top: 0.5rem;
	}

	.detail-group {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.detail-group label {
		font-size: 0.85rem;
		opacity: 0.7;
		font-weight: 500;
	}

	.detail-group select {
		padding: 0.5rem;
		border-radius: 6px;
		border: 1px solid rgba(255, 255, 255, 0.2);
		background: rgba(255, 255, 255, 0.1);
		color: inherit;
		font-size: 0.9rem;
		cursor: pointer;
		outline: none;
	}

	.detail-group select:focus {
		border-color: #667eea;
		background: rgba(255, 255, 255, 0.15);
	}
</style>
