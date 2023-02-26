import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';
import { getController, initController } from '../walletController';
import { createLCDClient, type ConnectedWallet } from '@terra-money/wallet-controller';
import { PUBLIC_CONTRACT_ADDR } from '$env/static/public';
// import {  Fee, MsgSend } from '@terra-money/terra.js';
import { Fee, MsgExecuteContract, MsgSend, Wallet } from '@terra-money/terra.js';
import { voterStore } from './voter';
import { getValueFromObservable } from '$lib/utils';

class WalletStore {
	state = {
		initialized: false,
		controller: null
	} as any;
	walletState = writable(null);
	connectedWallet: ConnectedWallet | null;
	LCDClient = createLCDClient({
		network: {
			name: 'testnet',
			chainID: 'pisco-1',
			lcd: 'https://pisco-lcd.terra.dev',
			walletconnectID: 0
		}
	});

	contractAddress: string = PUBLIC_CONTRACT_ADDR;

	constructor(state) {
		this.state = state;
		if (browser) {
			initController().then(() => {
				this.state.update((value) => {
					return {
						...value,
						initialized: true,
						controller: getController()
					};
				});
				getController()
					.states()
					.subscribe((_states) => {
						this.walletState.update(() => _states);
					});
				getController()
					.connectedWallet()
					.subscribe(async (_wallet) => {
						this.connectedWallet = _wallet;
						console.log(this.connectedWallet);
						if (this.connectedWallet) await this.refreshVoterStats();
						// reset if disconnect
					});
			});
		}
	}

	private async refreshVoterStats() {
		const voterInfo = await this.queryContract({
			voter_info: {
				addr: this.connectedWallet.walletAddress
			}
		});

		const isAdmin =
			(
				(await this.queryContract({
					is_admin: {
						addr: this.connectedWallet.walletAddress
					}
				})) as any
			).is_admin ?? false;
		voterStore.voterInfo.set(voterInfo);
		voterStore.isAdmin.set(isAdmin);
	}

	public async queryContract(query) {
		return this.LCDClient.wasm.contractQuery(this.contractAddress, query).catch(() => undefined);
	}

	public async executeContract(msg) {
		await this.connectedWallet.post({
			msgs: [new MsgExecuteContract(this.connectedWallet.walletAddress, this.contractAddress, msg)]
		});
	}
}

export const walletStore = new WalletStore(writable({ initialized: false, controller: null }));
