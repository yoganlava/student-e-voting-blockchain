<script>
	import { goto } from '$app/navigation';
	import { daysUntil } from '$lib/utils';
	import { faClock, faQuestionCircle, faScroll } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	const pollTypeIcons = {
		threshold: faQuestionCircle,
		percentage: faQuestionCircle,
		petition: faScroll
	};

	export let poll;

	const pollType = Object.keys(poll.kind)[0];

	const pollEndDate = new Date(poll.end_time / 1000000);
</script>

<div class="poll">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div class="poll__title" on:click={() => goto(`/poll/${poll.id}`)} title="{pollType[0].toUpperCase() + pollType.substring(1)} poll">
		<Fa icon={pollTypeIcons[pollType]} />
		<p>{poll.title}</p>
	</div>
	<div class="poll__footer">
		<div
			class={Date.now() <= pollEndDate.getTime() && daysUntil(new Date(), pollEndDate) <= 1
				? 'poll__footer-date-warning'
				: 'poll__footer-date'}
			title="End date of poll"
		>
			<Fa icon={faClock} />
			<p>{pollEndDate.toLocaleString().split(',')[0]}</p>
		</div>
		<p class="poll__footer-voter-count">Voters: {poll.votes?.total}</p>
	</div>
</div>

<style lang="scss">
	.poll {
		border: 2px solid black;
		border-radius: 0.4rem;
		// max-width: 500px;
		text-align: center;
		font-size: 1rem;
		margin: 1rem 0;

		&__title {
			padding: 0.5rem;
			font-size: 1.5rem;
			text-decoration: none;
			align-items: center;
			cursor: pointer;
			display: flex;
			& > p {
				margin-left: 0.5rem;
			}
		}

		&__footer {
			display: flex;
			max-height: 5rem;
			font-size: 12px;
			&-date {
				align-items: center;
				border-top: 1px solid black;
				border-right: 1px solid black;
				padding: 0.5rem;
				display: flex;

				& > p {
				margin-left: 0.5rem;
			}

				&-warning {
					@extend .poll__footer-date;
					background: orange;
					color: white;
				}
			}

			&-voter-count {
				border-top: 1px solid black;
				padding: 0.5rem;
				width: 100%;
			}
		}
	}
</style>
