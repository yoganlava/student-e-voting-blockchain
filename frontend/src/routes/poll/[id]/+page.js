import { walletStore } from '$lib/stores/wallet';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	try {
		return {
			poll: await walletStore.queryContract({
				poll: {
					poll_id: parseInt(params.id, 10)
				}
			})
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
