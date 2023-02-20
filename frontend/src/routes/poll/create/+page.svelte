<script>

let form = {};

function createPoll() {
    
} 

</script>

<div class="create-poll-page">
	<div class="create-poll-container">
		<div class="create-poll-container__element">
			<p>Title</p>
			<input type="text" />
		</div>
		<div class="create-poll-container__element">
			<p>Description</p>
			<textarea class="create-poll-container__element-textarea"/>
		</div>
        <div class="create-poll-container__element">
            Poll Type
            <label>
                <input type="radio" name="choice" checked disabled/>
                Yes/No
            </label>
            <label>
                <input type="radio" name="choice" disabled/>
                Threshold
            </label>
            <label>
                <input type="radio" name="choice" disabled/>
                Petition
            </label>
        </div>
		<button class="create-poll-container__button btn" on:click={() => {}}>Create</button>
	</div>
</div>

<style lang="scss">
	.create-poll-container {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

		&__element {
			margin: auto;

            &-textarea {
                width: 80%;
                min-height: 10rem;
                resize: none;
            }
		}

		&__button {
			margin-top: 1rem;
		}
	}
</style>
