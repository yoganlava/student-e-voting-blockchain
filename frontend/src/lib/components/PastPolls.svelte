<script>
	import PollCard from '$lib/components/PollCard.svelte';
</script>

<div class="past-polls">
	<h1>Past Polls:</h1>
	<div class="past-polls__list">
		<PollCard />
		<PollCard />
	</div>
</div>

<style lang="scss">
	.past-polls {
		&__list {
			display: flex;
		}
	}
</style>
