<script>
	import { walletStore } from '$lib/stores/wallet';
	import { onMount } from 'svelte';

	const tokenBalance = walletStore.tokenBalance;
</script>

<span class="token-balance">
	{$tokenBalance} SVT
</span>

<style lang="scss">
	.token-balance {
		color: white;
		background-color: black;
		border-radius: 0.2rem;
		padding: 0.5rem;
	}
</style>
