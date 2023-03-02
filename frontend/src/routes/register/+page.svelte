<script>
	import WalletConnect from '$lib/components/WalletConnect.svelte';
	import { walletStore } from '$lib/stores/wallet';
	import { toastDanger, toastSuccess } from '$lib/utils';
	import { WalletStatus } from '@terra-money/wallet-provider';
	// import { getContext } from 'svelte';

	let form = {
		email: '',
		name: '',
		student_code: 0
	};
	// const walletState = getContext('wallet')?.walletState;
	const walletState = walletStore.walletState;

	// TODO: Check if already registered

	async function register() {
		if ($walletState?.status == WalletStatus.WALLET_NOT_CONNECTED) {
			return toastDanger('Wallet not connected');
		}

		const response = await walletStore.executeContract({
			register_voter: form
		});

		toastSuccess('Voter registered!');
	}
</script>

<div class="register-page">
	<div class="register-container">
		<div class="register-container__element">
			<p>Connect Wallet</p>
			{#if !$walletState || $walletState?.status == WalletStatus.INITIALIZING}
				Loading...
			{:else if $walletState?.status == WalletStatus.WALLET_NOT_CONNECTED}
				<WalletConnect />
			{:else if $walletState?.status == WalletStatus.WALLET_CONNECTED}
				Wallet Connected
			{/if}
		</div>
		<div class="register-container__element">
			<p>Email</p>
			<input type="text" bind:value={form.email} />
		</div>
		<div class="register-container__element">
			<p>Full Name</p>
			<input type="text" bind:value={form.name} />
		</div>
		<div class="register-container__element">
			<p>Student Code</p>
			<input type="number" bind:value={form.student_code} />
		</div>
		<button class="register-container__button btn" on:click={register}>Register</button>
	</div>
</div>

<style lang="scss">
	.register-container {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

		&__element {
			margin: auto;

			& > p::after {
				content: '*';
				color: red;
			}
		}

		&__button {
			margin-top: 1rem;
		}
	}
</style>
