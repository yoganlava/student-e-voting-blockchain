<script>
	import PollsList from "$lib/components/common/PollsList.svelte";

    export let data;
</script>
<div class="admin-page">
    <div>
		<h1>Pending Polls:</h1>
		<PollsList polls={data?.pendingPolls?.polls ?? []}/>
	</div>
</div>

<style lang="scss"> </style>