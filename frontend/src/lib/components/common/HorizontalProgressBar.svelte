<script>
    /**
	 * @type {{ label: String, value: Number; max: Number; color: String }}
	 */
     export let stats;
</script>

<div class="horizontal-progress-bar-container">
    <label for="progress-{stats.label}">{stats.label}</label>
    <progress id="progress-{stats.label}" value={stats.value} max={stats.max} style="--progress-color: {stats.color}"/>
</div>

<style lang="scss">
    // todo make name shorter
    .horizontal-progress-bar-container {
        border-radius: 0.5rem;
        
        label {
            display: block;
        }

        progress {
            height: 2rem;
            appearance: none;

            &::-webkit-progress-bar {
                border-radius: 0.7rem;
            }

            &::-webkit-progress-value {
                border-radius: 0.7rem;
                background-color: var(--progress-color);
            }
        }
    }
</style>