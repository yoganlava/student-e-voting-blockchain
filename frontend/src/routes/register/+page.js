import { walletStore } from '$lib/stores/wallet';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {

    if (!walletStore.connectedWallet) return;

    // const contractRes = await walletStore.queryContract({
    //     VoterInfo: {
    //         addr: walletStore.connectedWallet.walletAddress
    //     }
    // });
};