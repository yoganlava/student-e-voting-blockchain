import toast from 'svelte-french-toast';
import http from './http';
import { walletStore } from './stores/wallet';

export async function sleep(timeout: number) {
	await new Promise((resolve) => setTimeout(resolve, timeout));
}

export function getValueFromObservable(observable) {
	let temp;
	observable.subscribe((value) => (temp = value));
	return temp;
}

export function toastSuccess(message: string) {
	toast.success(message, {
		style: 'color: mintcream; background: rgb(72,187,120)'
	});
}

export function toastDanger(message: string) {
	toast.error(message, {
		style: 'color: mintcream; background: rgb(212, 48, 32)'
	});
}

export async function getTxLogAttribute(txHash, attributeKey) {
	const response = await walletStore.LCDClient.tx.txInfo(txHash);
	const event = response.logs[0].events.find((event) => event.type === 'wasm');
	return event.attributes.find((attribute) => attribute.key === attributeKey).value;
}

export function daysUntil(from, to) {
	let difference = from.getTime() - to.getTime();
	return Math.ceil(difference / (1000 * 3600 * 24));
}
