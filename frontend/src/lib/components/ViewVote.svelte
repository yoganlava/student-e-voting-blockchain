<script>
	export let vote;
</script>

<div class="view-vote">
	<p>Voter Address: {vote.voter_addr}</p>
	<p>Poll ID: <a href="/poll/{vote.poll_id}">{vote.poll_id}</a></p>
	<p>Vote Type: {vote.decrypted_vote_kind}</p>
	<p>Vote Tracker: {vote.decrypted_vote_tracker}</p>
	<p>Malformed: {vote.malformed}</p>
</div>

<style lang="scss">
	.view-vote {
		margin: 1rem auto;
		text-align: center;
		max-width: 30rem;
		border: 2px solid black;
		border-radius: 1rem;
		padding: 1rem;
	}
</style>
