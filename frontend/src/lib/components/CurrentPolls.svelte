<script>
	import Poll from '$lib/components/Poll.svelte';
</script>

<div class="current-polls">
	<h1>Current Polls:</h1>
	<div class="current-polls__list">
		<Poll />
		<Poll />
	</div>
</div>

<style lang="scss">
	.current-polls {
		&__list {
			display: flex;
		}
	}
</style>
