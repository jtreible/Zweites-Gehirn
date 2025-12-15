<script lang="ts">
	import { onMount } from 'svelte';
	import TaskList from '$components/tasks/TaskList.svelte';
	import QuickCapture from '$components/tasks/QuickCapture.svelte';
	import { tasks, loadTasks } from '$stores/tasks';

	onMount(async () => {
		await loadTasks();
	});
</script>

<div class="app-container">
	<header>
		<h1>Zweites Gehirn</h1>
		<p class="subtitle">Your ADHD-Friendly Second Brain</p>
	</header>

	<div class="main-content">
		<QuickCapture />
		<TaskList />
	</div>
</div>

<style>
	.app-container {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		padding: 2rem;
		gap: 2rem;
	}

	header {
		text-align: center;
	}

	h1 {
		font-size: 2.5rem;
		margin-bottom: 0.5rem;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		font-size: 1.1rem;
		opacity: 0.7;
	}

	.main-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		max-width: 800px;
		margin: 0 auto;
		width: 100%;
	}
</style>
