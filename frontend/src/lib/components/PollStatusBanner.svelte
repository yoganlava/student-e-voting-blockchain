<script>
	import {
		faCircleCheck,
		faCircleExclamation,
		faCirclePause,
		faCircleXmark
	} from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	const states = {
		active: {
			icon: faCircleExclamation,
			prefix: 'is'
		},
		pending: {
			icon: faCirclePause,
			prefix: 'is'
		},
		passed: {
			icon: faCircleCheck,
			prefix: 'has'
		},
		rejected: {
			icon: faCircleXmark,
			prefix: 'has been'
		}
	};

	export let status;
</script>

<div class="banner banner-{status}">
	<Fa icon={states[status].icon} />
	<div class="banner__message">The current poll {states[status].prefix} {status}</div>
</div>

<style lang="scss">
	.banner {
		grid-area: banner;
		border-radius: 0.5rem;
		padding: 1rem;
		font-weight: bold;
		margin-bottom: 0.5rem;
		display: flex;
        align-items: center;

		&__message {
			align-self: center;
			margin-left: 1rem;
		}

		&-active {
			background-color: rgb(190, 190, 190);
			color: rgb(37, 37, 37);
		}

		&-pending {
			background-color: rgb(188, 209, 253);
			color: rgb(60, 82, 153);
		}

		&-passed {
			background-color: rgb(166, 233, 194);
			color: rgb(60, 153, 98);
		}

		&-rejected {
			background-color: rgb(223, 140, 132);
			color: rgb(172, 38, 25);
		}
	}
</style>
