<script>
	import PollsList from "./common/PollsList.svelte";


	export let header, polls;
</script>

<div class="poll-column">
	<h1>{header}</h1>
	<PollsList {polls} />
</div>

<style lang="scss">
    
</style>
