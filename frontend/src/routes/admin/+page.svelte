<script>
	import PollsList from "$lib/components/common/PollsList.svelte";
	import GiftSection from "$lib/components/GiftSection.svelte";

    export let data;
</script>
<div class="admin-page">
	<GiftSection/>
    <div class="admin-page__polls">
		<PollsList title="Pending Polls" polls={data?.pendingPolls?.polls}/>
	</div>
</div>

<style lang="scss">
	.admin-page {
		&__polls {
			display: flex;
		}
	}
</style>