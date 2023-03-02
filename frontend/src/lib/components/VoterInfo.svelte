<script>
    export let voter;
</script>

<div class="voter-info">
    <h1>Info</h1>
    {#if voter}
    <ul>
        <li>Address: {voter.addr}</li>
        <li>Name: {voter.name}</li>
        <li>Email: {voter.email}</li>
        <li>Student Code: {voter.student_code}</li>
    </ul>
    {:else}
        <p>Not Registered</p>
    {/if}
</div>

<style lang="scss">
    .voter-info {
        border: 2px solid black;
        border-radius: 1rem;
        padding: 2rem;
    }
</style>