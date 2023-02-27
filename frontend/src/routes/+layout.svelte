<script>
	import NavBar from '$lib/components/NavBar.svelte';
	import '$lib/styles/main.scss';
</script>

<main>
	<NavBar />
	<div class="page-container">
		<slot />
	</div>
</main>
