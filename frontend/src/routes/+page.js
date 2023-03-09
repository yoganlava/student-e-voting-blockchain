import { walletStore } from '$lib/stores/wallet';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	//TODO:? Combine into one 
	try {
		return {
			currentPolls: await walletStore.queryContract({
				polls: {
					status: 'active'
				}
			}),
			pendingPolls: await walletStore.queryContract({
				polls: {
					status: 'pending'
				}
			}),
			passedPolls: await walletStore.queryContract({
				polls: {
					status: 'passed'
				}
			}),
			rejectedPolls: await walletStore.queryContract({
				polls: {
					status: 'rejected'
				}
			})
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
