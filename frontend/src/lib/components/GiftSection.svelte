<script>
	import { PUBLIC_CONTRACT_ADDR, PUBLIC_TOKEN_ADDR } from "$env/static/public";
	import { walletStore } from "$lib/stores/wallet";

	let form = {
		receiver: "",
		message: ""
	}, amount = 0

	async function gift() {
		console.log(form)

		await walletStore.executeContract(
			{
				send: {
					contract: PUBLIC_CONTRACT_ADDR,
					amount: String(amount),
					msg: Buffer.from(
						JSON.stringify({
							gift_voter: form
						})
					).toString('base64')
				}
			},
			PUBLIC_TOKEN_ADDR
		)
	}
</script>

<div class="gift-section">
	<h1>Gifting Area</h1>
	<div class="gift-section__form">
		<!-- Make dropdown -->
		<div class="gift-section__form-input">
			<p>User Address:</p>
			<input bind:value={form.receiver} type="text" />
		</div>
		<div class="gift-section__form-input">
			<p>Amount:</p>
			<input bind:value={amount} type="number" />
		</div>
		<div class="gift-section__form-input">
			<p>Reason:</p>
			<textarea bind:value={form.message} />
		</div>
	</div>
	<button class="btn" on:click={gift}>Gift</button>
</div>

<style lang="scss">
	.gift-section {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

		&__form {
			display: table;
			margin: auto;
			border-spacing: 1rem;


			&-input {
				display: table-row;
				p {
					display: table-cell;
				}
				input {
					width: 100%;
					display: table-cell;
				}
				textarea {
					display: table-cell;
					min-width: 15rem;
					resize: none;
				}
			}
		}

		// &__info {
		// 	font-size: 1.5rem;
		// 	font-weight: bold;
		// }

		// &__choices {
		// 	margin: 1rem;
		// }

		// &__button {
		// 	margin-top: 1rem;
		// }

		// &__tracker {
		// 	&-message {
		// 		font-size: 13;
		// 		margin-bottom: 1rem;
		// 	}
		// }
	}
</style>
