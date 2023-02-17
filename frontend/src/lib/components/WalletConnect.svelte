<script>
	import { ConnectType } from '@terra-money/wallet-controller';
	import { onMount } from 'svelte';
	import { faWallet } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { walletStore } from '$lib/stores/wallet';
	const walletState = walletStore.walletState;
	const wallet = walletStore.state;
	onMount(() => {
		console.log($walletState);
	});

	async function connectWallet() {
		if (!$walletState || $walletState?.status == 'INITIALIZING') {
			return;
		} else if ($walletState?.status === 'WALLET_NOT_CONNECTED') {
			$wallet.controller.connect(ConnectType.EXTENSION);
		} else {
			$wallet.controller.disconnect();
		}
	}
</script>

<button class="wallet-connect" on:click={connectWallet}>
	<Fa icon={faWallet} style="margin-right:1rem;" />
	{#if !$walletState || $walletState?.status == 'INITIALIZING'}
		Initializing...
	{:else if $walletState?.status === 'WALLET_NOT_CONNECTED'}
		Connect Extension
	{:else if $walletState?.status === 'WALLET_CONNECTED'}
		Disconnect
	{/if}
</button>

<style lang="scss">
	.wallet-connect {
		appearance: none;
		padding: 0.5rem;
		border-radius: 10rem;
		cursor: pointer;
	}
</style>
