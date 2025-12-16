<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	$: currentPath = $page.url.pathname;
</script>

<div class="app-layout">
	<nav class="main-nav">
		<a href="/" class:active={currentPath === '/'}>📝 Tasks</a>
		<a href="/kanban" class:active={currentPath === '/kanban'}>📊 Kanban</a>
	</nav>
	<main>
		<slot />
	</main>
</div>

<style>
	.app-layout {
		width: 100%;
		height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.main-nav {
		display: flex;
		gap: 1rem;
		padding: 1rem 2rem;
		background: rgba(255, 255, 255, 0.05);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.main-nav a {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		text-decoration: none;
		color: rgba(255, 255, 255, 0.7);
		transition: all 0.2s;
	}

	.main-nav a:hover {
		background: rgba(255, 255, 255, 0.1);
		color: rgba(255, 255, 255, 0.9);
	}

	.main-nav a.active {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: white;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}
</style>
