<script>
	import { PUBLIC_MIXNET_URL } from '$env/static/public';
	import http from '$lib/http';
	import { walletStore } from '$lib/stores/wallet';
	import { toastDanger } from '$lib/utils';
	import { encrypt } from 'eciesjs';

	export let data;

	let form = {
		pollID: data.poll.id,
		tracker: 0,
		vote: 'up_vote'
	};

	function generateTracker() {
		form.tracker = Math.floor(Math.random() * 100000000);
	}

	async function encryptVote({ pollID, tracker, vote }) {
		const { keys } = await http(`${PUBLIC_MIXNET_URL}/mixnet/keys?pollID=${pollID}`, {}, 'GET');

		return keys.reduce((text, publicKey) => encrypt(publicKey, text), `${vote}.${tracker}`)
	}

	async function registerVote() {
		if (!form.tracker) {
			toastDanger('Generate a valid tracker');
			return;
		}

		console.log(form);
		const encryptedVote = await encryptVote(form);
		await walletStore.executeContract({
			cast_vote: {
				poll_id: data.poll.id,
				encrypted_vote: new Array(...encryptedVote)
			}
		});
	}
</script>

<div class="vote-page">
	<div class="vote-page__info">
		{data.poll.title}
	</div>
	<div class="vote-page__choices">
		<label>
			<input type="radio" bind:group={form.vote} name="choice" value="up_vote" />
			Yes
		</label>
		<label>
			<input type="radio" bind:group={form.vote} name="choice" value="down_vote" />
			No
		</label>
	</div>
	<div class="vote-page__tracker">
		<p class="vote-page__tracker-message">
			Press the button to randomly generate an 8 digit tracker code. This will be used to verify the
			vote. You can also type it in.
		</p>
		<input type="number" value={form.tracker} min="1000000" max="99999999" required />
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
