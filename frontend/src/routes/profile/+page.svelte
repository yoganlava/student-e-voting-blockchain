<script>
	import PollsList from '$lib/components/common/PollsList.svelte';
import PollCard from '$lib/components/PollCard.svelte';
	import VoterInfo from '$lib/components/VoterInfo.svelte';
	import { voterStore } from '$lib/stores/voter';

	const voterInfo = voterStore.voterInfo;

	export let data;
</script>

<div class="profile-page">
	<div class="profile-page__info">
		<p class="profile-page__info-welcome">Welcome {$voterInfo.name}</p>
			<VoterInfo voter={$voterInfo} />
		<!-- <div style="display: flex;"> -->
			<PollsList title="Participated Polls" polls={data?.participatedPolls}/>
		<!-- </div> -->
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
