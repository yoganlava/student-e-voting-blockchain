<script>
	import PollCard from '$lib/components/PollCard.svelte';
</script>

<div class="current-polls">
	<h1>Current Polls:</h1>
	<div class="current-polls__list">
		<PollCard />
		<PollCard />
	</div>
</div>

<style lang="scss">
	.current-polls {
		&__list {
			display: flex;
		}
	}
</style>
