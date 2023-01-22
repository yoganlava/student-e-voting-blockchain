<script>
	import HorizontalProgressBar from "./common/HorizontalProgressBar.svelte";

    let stats = {
        type: "yes_no",
        yes: 234567,
        no: 128945
    }
</script>

<div class="poll-stats">
    <div class="poll-stats__stat">
        <HorizontalProgressBar stats={{
            label: "Yes",
            value: stats.yes,
            max: stats.yes + stats.no,
            // TODO find better way to modularise colours
            // just put colors in theme and then change class
            color: "#7FFA8A"
        }}/>
    </div>
    <div class="poll-stats__stat">
        <HorizontalProgressBar stats={{
            label: "No",
            value: stats.no,
            max: stats.yes + stats.no,
            color: "#3B46F1"
        }}/>
    </div>
</div>

<style lang="scss">
    .poll-stats {
        &__stat {

        }
    }
</style>