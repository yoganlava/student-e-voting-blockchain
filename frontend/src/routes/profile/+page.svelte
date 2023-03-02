<script>
	import PollCard from '$lib/components/PollCard.svelte';
	import VoterInfo from '$lib/components/VoterInfo.svelte';
	import { voterStore } from '$lib/stores/voter';

	const voterInfo = voterStore.voterInfo;
</script>

<div class="profile-page">
	<div class="profile-page__info">
		{#if !$voterInfo}
			<p>Not Registered</p>
		{:else}
			<p class="profile-page__info-welcome">Welcome {$voterInfo.name}</p>
			<VoterInfo voter={$voterInfo} />
			<h1>Participated Polls:</h1>
			<div style="display: flex;">
				<PollCard />
				<PollCard />
			</div>
		{/if}
	</div>
</div>

<style lang="scss">
	.profile-page {
		&__info {
			margin: 0 5rem;
			&-welcome {
				font-size: 18px;
				font-weight: bold;
			}

			&-more-info {
				margin: 1rem 0;
			}
		}
	}
</style>
