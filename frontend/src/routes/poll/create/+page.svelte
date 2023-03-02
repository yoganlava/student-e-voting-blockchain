<script>
	import { PUBLIC_MIXNET_URL } from '$env/static/public';
	import http from '$lib/http';
	import { walletStore } from '$lib/stores/wallet';
	import { getTxLogAttribute, sleep, toastSuccess } from '$lib/utils';

	let form = {
		title: '',
		description: '',
		// start_time
		end_time: '',
		poll_type: 'threshold',
		votes_needed: 0,
		percentage_needed: 0
	};

	async function createPoll() {
		const formToSend = {
			title: form.title,
			description: form.description,
			end_time: String(new Date(form.end_time).getTime() * 1000000),
			kind: {
				[form.poll_type]: {
					votes_needed: form.votes_needed
				}
			}
		};

		const response = await walletStore.executeContract({
			create_poll: formToSend
		});
		console.log(response);

		if (response.success) toastSuccess('Poll Created!');

		await sleep(5000);

		const createdPollID = parseInt(
			await getTxLogAttribute(response.result.txhash, 'poll_id'),
			10
		);

		console.log(createdPollID);

		await http(
			`${PUBLIC_MIXNET_URL}/mixnet/notify/create`,
			{
				pollID: createdPollID
			},
			'POST'
		);

		console.log(response);
	}
</script>

<div class="create-poll-page">
	<div class="create-poll-container">
		<div class="create-poll-container__element">
			<p>Title</p>
			<input type="text" bind:value={form.title} />
		</div>
		<div class="create-poll-container__element">
			<p>Description</p>
			<textarea class="create-poll-container__element-textarea" bind:value={form.description} />
		</div>
		<div class="create-poll-container__element">
			<p>End Time</p>
			<input type="date" bind:value={form.end_time} />
		</div>
		<div class="create-poll-container__element">
			<p>Poll Type</p>
			<label>
				<input type="radio" name="choice" checked disabled />
				Yes/No
			</label>
			<label>
				<input type="radio" name="choice" disabled />
				Threshold
			</label>
			<label>
				<input type="radio" name="choice" disabled />
				Petition
			</label>
		</div>
		<div class="create-poll-container__element">
			<p>Votes Needed to pass</p>
			<input type="number" bind:value={form.votes_needed} />
		</div>
		<button class="create-poll-container__button btn" on:click={createPoll}>Create</button>
		<div class="create-poll-container__info">
			<p>Fee: 10 SVT</p>
		</div>
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

			margin: 0.5rem 0;

			> p::after {
				content: '*';
				color: red;
			}

			&-textarea {
				width: 80%;
				min-height: 10rem;
				resize: none;
			}
		}

		&__button {
			margin-top: 1rem;
		}

		&__info {
			margin-top: 0.5rem;
		}
	}
</style>
