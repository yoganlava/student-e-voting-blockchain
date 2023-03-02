<!-- TODO Charts -->
<script>
	import { Pie } from 'svelte-chartjs';
	export let poll;
    import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale } from 'chart.js';

	ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

	const data = {
		labels: ['Yes', 'No', 'Malformed'],
		datasets: [
			{
				data: [poll.votes.up_votes, poll.votes.down_votes, poll.votes.malformed_votes],
                backgroundColor: [
                    "green",
                    "red",
                    "black"
                ]
			}
		]
	};
</script>

<div class="votes-breakdown">
	<h1>Total Votes: {poll.votes.total}</h1>
	{#if poll.status == 'passed' || poll.status == 'rejected'}
		<div class="votes-breakdown__stats">
			<Pie {data} options={{ responsive: true }} />
		</div>
	{:else if Object.keys(poll.kind)[0] != 'petition'}
		<h2>Please wait for the votes to be tallied to see the break down</h2>
	{/if}
</div>

<style lang="scss">
	.votes-breakdown {
		&__stats {
		}
	}
</style>
