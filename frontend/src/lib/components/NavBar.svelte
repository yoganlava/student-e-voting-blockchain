<script>
	import { voterStore } from '$lib/stores/voter';
	import TokenBalance from './TokenBalance.svelte';
	import WalletConnect from './WalletConnect.svelte';
</script>

<ul class="nav-bar">
	<a class="nav-bar__logo" href="/">Surrey E-Vote</a>
	<div class="nav-bar__items">
		<a href="/profile"><li>Profile</li></a>
		<a href="/poll/create"><li>Create Poll</li></a>
		<a href="/register"><li>Register</li></a>
		{#if voterStore.isAdmin}
			<a href="/admin"><li>Admin</li></a>
		{/if}
		<TokenBalance />
		<WalletConnect />
	</div>
</ul>

<style lang="scss">
	.nav-bar {
		display: flex;
		justify-content: space-between;
		list-style-type: none;
		overflow: hidden;
		// make theme file with vars
		background-color: rgb(80, 80, 80);
		a {
			text-decoration: none;
			color: white;
		}

		&__logo {
			font-size: 2rem;
			padding: 0.5rem;
			font-weight: bold;
		}

		&__items {
			display: inline;
			align-items: center;
			a {
				height: 100%;
				display: inline-block;
				margin-right: 0.5rem;

				// TODO: Bad fix
				li {
					margin-top: 1rem;
				}

				&:hover {
					background-color: rgb(70, 70, 70);
				}
			}
		}
	}
</style>
