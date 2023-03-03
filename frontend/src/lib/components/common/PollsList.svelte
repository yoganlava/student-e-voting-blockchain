<script>
	import PollCard from '$lib/components/PollCard.svelte';

	export let polls;
</script>

<div class="polls-list">
	{#each polls as poll (poll.id)}
		<PollCard {poll} />
		<!-- <PollCard {poll} /> -->
	{/each}
</div>

<style lang="scss">
	.polls-list {
		display: flex;
	}
</style>
