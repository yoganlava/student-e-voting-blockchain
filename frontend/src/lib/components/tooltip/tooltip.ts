import Tooltip from './ToolTip.svelte';

export function tooltip(element) {
	let title;
	let tooltipComponent;
	function mouseOver(event) {
		title = element.getAttribute('title');
		element.removeAttribute('title');

		tooltipComponent = new Tooltip({
			props: {
				title: title,
				x: event.pageX,
				y: event.pageY
			},
			target: document.body
		});
	}
	function mouseMove(event) {
		tooltipComponent.$set({
			x: event.pageX,
			y: event.pageY
		});
	}
	function mouseLeave() {
		tooltipComponent.$destroy();
		element.setAttribute('title', title);
	}

	element.addEventListener('mouseover', mouseOver);
	element.addEventListener('mouseleave', mouseLeave);
	element.addEventListener('mousemove', mouseMove);

	return {
		destroy() {
			element.removeEventListener('mouseover', mouseOver);
			element.removeEventListener('mouseleave', mouseLeave);
			element.removeEventListener('mousemove', mouseMove);
		}
	};
}
