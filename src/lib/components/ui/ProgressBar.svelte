<script lang="ts">
	import type { SubtaskProgress } from '$types/task';

	export let progress: SubtaskProgress;
</script>

<div class="progress-container">
	<div class="progress-text">
		{progress.completed}/{progress.total} completed
	</div>
	<div class="progress-bar">
		<div class="progress-fill" style="width: {progress.percentage}%"></div>
	</div>
	<div class="progress-percentage">{Math.round(progress.percentage)}%</div>
</div>

<style>
	.progress-container {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0;
	}

	.progress-text {
		font-size: 0.85rem;
		color: #666;
		white-space: nowrap;
	}

	.progress-bar {
		flex: 1;
		height: 8px;
		background: #e0e0e0;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		transition: width 0.3s ease;
		border-radius: 4px;
	}

	.progress-percentage {
		font-size: 0.85rem;
		font-weight: 600;
		color: #667eea;
		min-width: 45px;
		text-align: right;
	}
</style>
