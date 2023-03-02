import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';
import { getController, initController } from '../walletController';
import { createLCDClient, type ConnectedWallet } from '@terra-money/wallet-controller';
import { PUBLIC_CONTRACT_ADDR, PUBLIC_TOKEN_ADDR } from '$env/static/public';
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
	tokenAddress: string = PUBLIC_TOKEN_ADDR;
	tokenBalance: Writable<string> = writable('0');

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
						if (this.connectedWallet) {
							await this.refreshVoterStats();
							await this.refreshBalance();
						}
						// reset if disconnect
					});
			});
		}
	}

	private async refreshVoterStats() {
		const [voterInfo, adminRes] = await Promise.all([
			this.queryContract({
				voter_info: {
					addr: this.connectedWallet.walletAddress
				}
			}),
			this.queryContract({
				is_admin: {
					addr: this.connectedWallet.walletAddress
				}
			})
		]);
		voterStore.voterInfo.set(voterInfo);
		voterStore.isAdmin.set((adminRes as any).is_admin ?? false);
	}

	public async queryContract(query) {
		return this.LCDClient.wasm.contractQuery(this.contractAddress, query).catch(() => undefined);
	}

	public async refreshBalance() {
		this.tokenBalance.set(
			(
				(await this.LCDClient.wasm
					.contractQuery(this.tokenAddress, {
						balance: {
							address: this.connectedWallet.walletAddress
						}
					})
					.catch(() => undefined)) as any
			).balance
		);
	}

	public async executeContract(msg) {
		return await this.connectedWallet.post({
			msgs: [new MsgExecuteContract(this.connectedWallet.walletAddress, this.contractAddress, msg)]
		});
	}
}

export const walletStore = new WalletStore(writable({ initialized: false, controller: null }));
