import { voterStore } from '$lib/stores/voter';
import { walletStore } from '$lib/stores/wallet';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    let isAdmin;

    voterStore.isAdmin.subscribe((_isAdmin) => {isAdmin = _isAdmin});


    if (!isAdmin) throw redirect(307, '/');

	try {
		return {
			pendingPolls: await walletStore.queryContract({
				polls: {
					status: 'pending'
				}
			}),
		};
	} catch (e) {
		console.log(e);
		return {};
	}
}
