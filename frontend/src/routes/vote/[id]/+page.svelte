<script>
    let poll = {
        title: "Should Udeshya be king?"
    }

	function registerVote() {}
</script>

<div class="vote-page">
    <div class="vote-page__info">
        {poll.title}
    </div>
	<div class="vote-page__choices">
		<label>
			<input type="checkbox" />
			Yes
		</label>
		<label>
			<input type="checkbox" />
			No
		</label>
	</div>
	<button class="register-container__button btn" on:click={registerVote}>Register Vote</button>
</div>

<style lang="scss">
	.vote-page {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

        &__info{
            font-size: 1.5rem;
			font-weight: bold;
		}

		&__choices {
			margin: 1rem;
		}

		&__button {
			margin-top: 1rem;
		}
	}
</style>
