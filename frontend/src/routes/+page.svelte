<script>
	import PollsList from "$lib/components/common/PollsList.svelte";

	export let data;

	console.log(data)
</script>

<div class="main-page">
	<PollsList title="Current Polls" polls={data?.currentPolls?.polls}/>
	<PollsList title="Pending Polls" polls={data?.pendingPolls?.polls}/>
	<PollsList title="Passed Polls" polls={data?.passedPolls?.polls}/>
	<PollsList title="Rejected Polls" polls={data?.rejectedPolls?.polls}/>
</div>

<style lang="scss">
	.main-page {
		display: flex;
	}
</style>
