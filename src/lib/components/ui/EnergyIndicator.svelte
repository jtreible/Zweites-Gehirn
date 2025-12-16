<script lang="ts">
	export let level: string | null = null;

	const getIndicator = (level: string) => {
		switch(level?.toLowerCase()) {
			case 'low': return { icon: '🔋', label: 'Low Energy', count: 1 };
			case 'medium': return { icon: '🔋', label: 'Medium Energy', count: 2 };
			case 'high': return { icon: '🔋', label: 'High Energy', count: 3 };
			default: return null;
		}
	};

	$: indicator = level ? getIndicator(level) : null;
</script>

{#if indicator}
	<span class="energy-indicator" title="Required Energy: {indicator.label}">
		{indicator.icon.repeat(indicator.count)}
	</span>
{/if}

<style>
	.energy-indicator {
		display: inline-flex;
		align-items: center;
		font-size: 0.85rem;
		line-height: 1;
	}
</style>
