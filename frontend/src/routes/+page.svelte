<script>
	import PollsList from "$lib/components/common/PollsList.svelte";

	export let data;

	console.log(data)
</script>

<div class="main-page">
	<div>
		<h1>Current Polls:</h1>
		<PollsList polls={data.currentPolls.polls}/>
	</div>
	<div>
		<h1>Past Polls:</h1>
		<PollsList polls={data.pastPolls.polls}/>
	</div>
</div>

<style lang="scss">
	.main-page {

	}
</style>
