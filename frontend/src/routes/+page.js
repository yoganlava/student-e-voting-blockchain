import { walletStore } from '$lib/stores/wallet';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	try {
		return {
			currentPolls: await walletStore.queryContract({
				polls: {
					status: 'active'
				}
			}),
			pastPolls: await walletStore.queryContract({
				polls: {
					status: 'passed'
				}
			})
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
