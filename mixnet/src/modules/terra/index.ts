import { LCDClient } from "@terra-money/terra.js";
import { LocalTerra, Wallet } from "@terra-money/terra.js/dist/client";
import { Msg, MsgExecuteContract } from "@terra-money/terra.js/dist/core";
import { MnemonicKey } from "@terra-money/terra.js/dist/key";

let _client: LCDClient;
let _wallet: Wallet;

export function initialiseClientAndWallet() {
    _client = new LocalTerra();
    _wallet = _client.wallet(
        new MnemonicKey({
            mnemonic: process.env.WALLET_MNEMONIC,
        })
    );
    console.log("Initialised Wallet");
}

export function getWallet() {
    return _wallet;
}

export async function signAndBroadcastTx(...msgs: Msg[]) {
    const signedTx = await _wallet.createAndSignTx({
        msgs,
    });
    return await _client.tx.broadcast(signedTx);
}

export async function executeContractMessage(msg: Object) {
    return await signAndBroadcastTx(
        new MsgExecuteContract(
            getWallet().key.accAddress,
            process.env.CONTRACT_ADDR as string,
            msg
        )
    );
}

export async function queryContract(query: Object) {
    return await _client.wasm.contractQuery(
        process.env.CONTRACT_ADDR as string,
        query
    );
}
