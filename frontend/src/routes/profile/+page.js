import { walletStore } from '$lib/stores/wallet';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	//TODO:? Combine into one

	if (
		!walletStore.connectedWallet ||
		!(await walletStore.queryContract({
			voter_info: {
				addr: walletStore.connectedWallet.walletAddress
			}
		}))
	)
		throw redirect(307, '/');
	try {
		return {
			participatedPolls: await walletStore.queryContract({
				participated_polls: {
					addr: walletStore.connectedWallet.walletAddress
				}
			})
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
