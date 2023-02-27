<script>
	import { walletStore } from '$lib/stores/wallet';
	import { onMount } from 'svelte';

	let balance;

	onMount(async () => {
		console.log("Balance",
			await walletStore.LCDClient.wasm.contractQuery(walletStore.tokenAddress, {
				balance: {
					address: walletStore.connectedWallet.walletAddress
				}
			})
		);
		balance = await walletStore.queryBalance();
	});
</script>

<span class="token-balance">
	{!balance ? '-.-' : balance} SVT
</span>

<style lang="scss">
	.token-balance {
		color: white;
		background-color: black;
		border-radius: 0.2rem;
		padding: 0.5rem;
	}
</style>
