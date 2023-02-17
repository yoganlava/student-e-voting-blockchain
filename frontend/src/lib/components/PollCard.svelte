<script>
	import PollStats from "./PollStats.svelte";

    let poll = {
        title: "Should Udeshya be king?",
        voters: 1237
    }
</script>

<div class="poll">
    <a class="poll__title" href="#">{poll.title}</a>
    <p class="poll__voter-count">Voters: {poll.voters}</p>
    <div class="poll__stats">
        <PollStats/>
    </div>
</div>

<style lang="scss">
    .poll {
        background-color: rgb(34, 34, 34);
        color: white;
        border-radius: 0.8rem;
        max-width: 500px;
        display: grid;
        text-align: center;
        grid-template-areas: 
        ". title ."
        "voters voters voters"
        "stats stats stats";
        font-size: 1rem;
        margin: 1rem;

        &__title {
            padding: 0.5rem;
            font-size: 1.5rem;
            grid-area: title;
            text-decoration: none;
            color: white;
        }

        &__voter-count {
            padding: 0.5rem;
            grid-area: voters;
        }

        &__stats {
            padding: 0.5rem;
            grid-area: stats;
        }
    }
</style>