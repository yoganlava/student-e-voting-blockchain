<script>
	import PollCard from '$lib/components/PollCard.svelte';

	export let title, polls;
</script>

<div class="polls-list">
	<h1 class="polls-list__title">{title}</h1>
	<div class="polls-list__column">
		{#if polls?.length}
			{#each polls as poll (poll.id)}
				<PollCard {poll} />
			{/each}
		{:else}
			<h2 style="text-align: center;">No polls :(</h2>
		{/if}
	</div>
</div>

<style lang="scss">
	.polls-list {
		margin-left: 1rem;
		width: 300px;
		&__title {
			text-align: center;
		}
		&__column {
			display: block;
		}
	}
</style>
