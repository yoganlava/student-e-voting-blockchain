<script>
	let poll = {
		title: 'Should Udeshya be king?'
	};

    let form = {
        tracker: undefined
    };

    function generateTracker() {
        form.tracker = Math.floor(Math.random() * 100000000);
    }

	function registerVote() {}
</script>

<div class="vote-page">
	<div class="vote-page__info">
		{poll.title}
	</div>
	<div class="vote-page__choices">
		<label>
			<input type="radio" name="choice" checked />
			Yes
		</label>
		<label>
			<input type="radio" name="choice" />
			No
		</label>
	</div>
	<div class="vote-page__tracker">
		<p class="vote-page__tracker-message">
			Press the button to randomly generate an 8 digit tracker code. This will be used to verify the vote.
            You can also type it in.
		</p>
		<input type="number" value={form.tracker} min="1000000" max="99999999" required/>
        <button class="btn" on:click={generateTracker}>Generate</button>
	</div>
	<button class="vote-page__button btn" on:click={registerVote}>Register Vote</button>
</div>

<style lang="scss">
	.vote-page {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

		&__info {
			font-size: 1.5rem;
			font-weight: bold;
		}

		&__choices {
			margin: 1rem;
		}

		&__button {
			margin-top: 1rem;
		}

        &__tracker {
            &-message {
                font-size: 13;
                margin-bottom: 1rem;
            }
        }
	}
</style>
