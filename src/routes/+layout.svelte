<script lang="ts">
	import '../app.css';
</script>

<main>
	<slot />
</main>

<style>
	main {
		width: 100%;
		height: 100vh;
		display: flex;
		flex-direction: column;
	}
</style>
