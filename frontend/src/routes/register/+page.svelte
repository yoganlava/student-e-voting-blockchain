<script>
	import WalletConnect from '$lib/components/WalletConnect.svelte';
	import { walletStore } from '$lib/stores/wallet';
	import { WalletStatus } from '@terra-money/wallet-provider';
	// import { getContext } from 'svelte';

	// const walletState = getContext('wallet')?.walletState;
	const walletState = walletStore.walletState;


	// TODO: Check if already registered

	async function register() {}
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
			<input type="text" />
		</div>
		<div class="register-container__element">
			<p>Full Name</p>
			<input type="text" />
		</div>
		<button class="register-container__button btn" on:click={register}>Register</button>
	</div>
</div>

<style lang="scss">
	.register-container {
		margin: auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;

		&__element {
			margin: auto;
		}

		&__button {
			margin-top: 1rem;
		}
	}
</style>
