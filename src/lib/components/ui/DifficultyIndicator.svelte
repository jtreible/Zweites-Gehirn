<script lang="ts">
	export let level: number | null = null;

	const getLabel = (level: number) => {
		switch(level) {
			case 1: return 'Easy';
			case 2: return 'Medium';
			case 3: return 'Hard';
			case 4: return 'Very Hard';
			case 5: return 'Extreme';
			default: return '';
		}
	};
</script>

{#if level && level > 0 && level <= 5}
	<span class="difficulty-indicator" title="Difficulty: {getLabel(level)}">
		{'🌶️'.repeat(level)}
	</span>
{/if}

<style>
	.difficulty-indicator {
		display: inline-flex;
		align-items: center;
		font-size: 0.85rem;
		line-height: 1;
	}
</style>
