<script>
	import NavBar from "$lib/components/NavBar.svelte";
    import '$lib/styles/main.scss';
    import { setContext } from 'svelte';
	import { writable } from 'svelte/store';

    import { walletStore } from '$lib/stores/wallet';
	setContext('wallet', walletStore);
</script>


<main>
    <NavBar/>
    <div class="page-container">
        <slot/>
    </div>
</main>
