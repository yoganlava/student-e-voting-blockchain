<script>
	import { PUBLIC_MIXNET_URL } from '$env/static/public';
	import PollStatusBanner from '$lib/components/PollStatusBanner.svelte';
	import VotesBreakdown from '$lib/components/VotesBreakdown.svelte';
	import http from '$lib/http';
	import { walletStore } from '$lib/stores/wallet';
	

	export let data;

	async function tallyPoll() {
		await http(
			`${PUBLIC_MIXNET_URL}/mixnet/notify/finish`,
			{
				pollID: data.poll.id
			},
			'POST'
		);
	}

	async function closePoll() {
		await walletStore.executeContract({
			close_poll: {
				poll_id: data.poll.id
			}
		});
	}
</script>

<div class="poll">
	<PollStatusBanner status={data.poll.status}/>
	<div class="poll__title">{data.poll.title}</div>
	<div class="poll__description">
		{data.poll.description}
	</div>
	<div class="poll__side">
		<div class="poll__side-actions">
			<button class="btn">Vote</button>
			<button class="btn btn-danger" on:click={closePoll}>Close Poll</button>
			<button class="btn" on:click={tallyPoll}>Tally Poll</button>
		</div>
		<div class="poll__side-info">
			<ul>
				<li>Creator: {data.poll.creator}</li>
				<li>Poll Type: {Object.keys(data.poll.kind)[0].toUpperCase()}</li>
				<li>Status: {data.poll.status.toUpperCase()}</li>
				<li>
					Start Date: {new Date(data.poll.start_time / 1000000).toLocaleString().split(',')[0]}
				</li>
				<li>End Date: {new Date(data.poll.end_time / 1000000).toLocaleString().split(',')[0]}</li>
			</ul>
		</div>
	</div>
	<VotesBreakdown poll={data.poll}/>
</div>

<style lang="scss">
	.poll {
		margin: 1rem 5rem;
		display: grid;
		grid-template-areas:
			'banner banner banner'
			'title title side'
			'description description side'
			'. breakdown .';

		&__title {
			font-size: 1.5rem;
			font-weight: bold;
			grid-area: title;
		}

		&__description {
			grid-area: description;
			padding-top: 1rem;
			margin-right: 10rem;
		}

		&__side {
			grid-area: side;
			&-info {
				margin-top: 1rem;
			}
		}
	}
</style>
