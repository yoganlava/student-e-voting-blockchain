<script>
	import { goto } from '$app/navigation';
	import { daysUntil } from '$lib/utils';
	import { faClock, faQuestionCircle, faScroll } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { tooltip } from './tooltip/tooltip';

	const pollTypeIcons = {
		threshold: faQuestionCircle,
		percentage: faQuestionCircle,
		petition: faScroll
	};

	export let poll;

	const pollEndDate = new Date(poll.end_time / 1000000);
</script>

<div class="poll">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div class="poll__title" on:click={() => goto(`/poll/${poll.id}`)}>
		<Fa icon={pollTypeIcons[Object.keys(poll.kind)[0]]} />
		<p>{poll.title}</p>
	</div>
	<div class="poll__footer">
		<div
			class={Date.now() <= pollEndDate.getTime() && daysUntil(new Date(), pollEndDate) <= 1
				? 'poll__footer-date-warning'
				: 'poll__footer-date'}
            title="End date of poll"
            use:tooltip
		>
            <Fa icon={faClock}/>
			{pollEndDate.toLocaleString().split(',')[0]}
		</div>
		<p class="poll__footer-voter-count">Voters: {poll.votes?.total}</p>
	</div>
</div>

<style lang="scss">
	.poll {
		border: 2px solid black;
		border-radius: 0.4rem;
		max-width: 500px;
		text-align: center;
		font-size: 1rem;
		margin: 1rem;

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

			&-date {
				border-top: 1px solid black;
				border-right: 1px solid black;
				padding: 0.5rem;

				&-warning {
					@extend .poll__footer-date;
					background: orange;
					color: white;
				}
			}

			&-voter-count {
				border-top: 1px solid black;
				padding: 0.5rem;
			}
		}
	}
</style>
