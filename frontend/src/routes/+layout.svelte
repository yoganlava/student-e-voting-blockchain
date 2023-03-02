<script>
	import NavBar from '$lib/components/NavBar.svelte';
	import { Toaster } from 'svelte-french-toast';
	import '$lib/styles/main.scss';
</script>

<main>
	<Toaster />
	<NavBar />
	<div class="page-container">
		<slot />
	</div>
</main>
