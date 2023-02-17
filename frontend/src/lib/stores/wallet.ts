import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';
import { getController, initController } from '../walletController';
import { createLCDClient, type ConnectedWallet } from '@terra-money/wallet-controller';
import { PUBLIC_CONTRACT_ADDR } from "$env/static/public";
// import {  Fee, MsgSend } from '@terra-money/terra.js';
import { Fee, MsgSend, Wallet } from '@terra-money/terra.js';

class WalletStore {
	state = {
		initialized: false,
		controller: null
	} as any;
	walletState = writable(null);
	connectedWallet: ConnectedWallet | null;;
	LCDClient = createLCDClient({
		network: {
			name: 'testnet',
			chainID: 'bombay-12',
			lcd: 'https://bombay-lcd.terra.dev',
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
					.subscribe((_wallet) => {
						this.connectedWallet = _wallet;
					});
			});
		}
	}

	public queryContract(arg) {
		return this.LCDClient.wasm.contractQuery(this.contractAddress, { query: { ...arg } });
	}

	public executeContract(amount) {
		return this.connectedWallet.post({
			fee: new Fee(1000000, '200000uusd'),
			msgs: [
				new MsgSend(this.connectedWallet.walletAddress, this.contractAddress, {
					uusd: amount
				})
			]
		});
	}
}

export const walletStore = new WalletStore(writable({ initialized: false, controller: null }));