<script lang="ts">
	import KanbanBoard from '$lib/components/kanban/KanbanBoard.svelte';
</script>

<div class="kanban-page">
	<header>
		<h1>Kanban Board</h1>
		<p class="subtitle">Drag tasks between columns to update their status</p>
	</header>

	<KanbanBoard />
</div>

<style>
	.kanban-page {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	header {
		padding: 2rem 2rem 1rem 2rem;
		text-align: center;
	}

	h1 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		font-size: 1rem;
		opacity: 0.7;
	}
</style>
