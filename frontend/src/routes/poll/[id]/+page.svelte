<script>
	let poll = {
		title: 'Should Udeshya be king?',
		description:
			'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam. Sed nisi. Nulla quis sem at nibh elementum imperdiet. Duis sagittis ipsum. Praesent mauris. Fusce nec tellus sed augue semper porta. Mauris massa. Vestibulum lacinia arcu eget nulla. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Curabitur sodales ligula in libero. Sed dignissim lacinia nunc.\nCurabitur tortor. Pellentesque nibh. Aenean quam. In scelerisque sem at dolor. Maecenas mattis. Sed convallis tristique sem. Proin ut ligula vel nunc egestas porttitor. Morbi lectus risus, iaculis vel, suscipit quis, luctus non, massa. Fusce ac turpis quis ligula lacinia aliquet. Mauris ipsum. Nulla metus metus, ullamcorper vel, tincidunt sed, euismod in, nibh. ',
		voters: 1237,
		department: 'Computer Science',
		start_date: 'Now',
		end_date: 'Tomorrow'
	};
</script>

<div class="poll">
	<div class="poll__title">{poll.title}</div>
	<div class="poll__description">
		{poll.description}
	</div>
	<div class="poll__side">
		<div class="poll__side-actions">
			<button class="btn">Vote</button>
		</div>
		<div class="poll__side-info">
			<ul>
				<li>Dept: {poll.department}</li>
				<li>Start Date: {poll.start_date}</li>
				<li>End Date: {poll.end_date}</li>
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
