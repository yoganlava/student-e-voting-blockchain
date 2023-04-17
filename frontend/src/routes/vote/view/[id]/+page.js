import { walletStore } from '$lib/stores/wallet';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
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
			vote: await walletStore.queryContract({
				vote: {
					poll_id: parseInt(params.id, 10),
					addr: walletStore.connectedWallet.walletAddress
				}
			})
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
