<script>
	import PollsList from "$lib/components/common/PollsList.svelte";
	import PollColumn from "$lib/components/PollColumn.svelte";

	export let data;

	console.log(data)
</script>

<div class="main-page">
	<PollColumn header="Current Polls" polls={data.currentPolls.polls}/>
	<PollColumn header="Past Polls" polls={data.pastPolls.polls}/>
</div>

<style lang="scss">
	.main-page {
		
	}
</style>
