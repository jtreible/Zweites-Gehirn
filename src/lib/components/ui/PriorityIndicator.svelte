<script lang="ts">
	export let priority: string | null = null;

	const getIndicator = (priority: string) => {
		switch(priority?.toLowerCase()) {
			case 'urgent': return { icon: '🔴', label: 'Urgent', color: '#ef4444' };
			case 'high': return { icon: '🟠', label: 'High Priority', color: '#f59e0b' };
			case 'medium': return { icon: '🟡', label: 'Medium Priority', color: '#eab308' };
			case 'low': return { icon: '🟢', label: 'Low Priority', color: '#22c55e' };
			default: return null;
		}
	};

	$: indicator = priority ? getIndicator(priority) : null;
</script>

{#if indicator}
	<span class="priority-indicator" title="{indicator.label}" style="color: {indicator.color}">
		{indicator.icon}
	</span>
{/if}

<style>
	.priority-indicator {
		display: inline-flex;
		align-items: center;
		font-size: 0.9rem;
		line-height: 1;
	}
</style>
