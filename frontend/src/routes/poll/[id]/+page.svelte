<script>
	export let data;
</script>

<div class="poll">
	<div class="poll__title">{data.poll.title}</div>
	<div class="poll__description">
		{data.poll.description}
	</div>
	<div class="poll__side">
		<div class="poll__side-actions">
			<button class="btn">Vote</button>
			<button class="btn btn-danger">Close Poll</button>
		</div>
		<div class="poll__side-info">
			<ul>
				<li>Start Date: {data.poll.start_time}</li>
				<li>End Date: {data.poll.end_time}</li>
			</ul>
		</div>
	</div>
</div>

<style lang="scss">
	.poll {
		margin: 1rem 5rem;
		display: grid;
		grid-template-areas:
			'title title side'
			'description description side';

		&__title {
			font-size: 1.5rem;
			font-weight: bold;
			grid-area: title;
		}

		&__description {
			grid-area: description;
			padding-top: 1rem;
			margin-right: 10rem;
		}

		&__side {
			grid-area: side;
			&-info {
				margin-top: 1rem;
			}
		}
	}
</style>
