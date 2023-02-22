<script>
	import PollCard from "$lib/components/PollCard.svelte";


    const voterInfo = {
        addr: "terra1h0suyqek0zylfqhqrzfgw9sq7qeq6jqmpsfjgt",
        name: "Udeshya Gurung",
        email: "ug00031@surrey.ac.uk",
        student_code: 6577910
    };
</script>


<div class="profile-page">
    <div class="profile-page__info">
        <p class="profile-page__info-welcome">Welcome {voterInfo.name}</p>
        <ul class="profile-page__info-more-info">
            <li>Address: {voterInfo.addr}</li>
            <li>Email: {voterInfo.email}</li>
            <li>Student Code: {voterInfo.student_code}</li>
        </ul>
        <h1>Participated Polls:</h1>
        <div style="display: flex;">
            <PollCard />
            <PollCard />
        </div>
    </div>
</div>

<style lang="scss">
    .profile-page {
        margin: 0 5rem;
        &__info {
            &-welcome {
                font-size: 18px;
                font-weight: bold;
            }
            
            &-more-info {
                margin: 1rem 0;
            }
        }
    }
</style>
