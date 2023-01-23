import { LCDClient } from "@terra-money/terra.js";
import { LocalTerra, Wallet } from "@terra-money/terra.js/dist/client";
import { Msg } from "@terra-money/terra.js/dist/core";
import { MnemonicKey } from "@terra-money/terra.js/dist/key";

let _client: LCDClient;
let _wallet: Wallet;

export function initialiseClientAndWallet() {
    if (!_client) _client = new LocalTerra();
    if (!_wallet) _wallet = _client.wallet(
        new MnemonicKey(
            {
                mnemonic: process.env.WALLET_MNEMONIC
            }
        )
    ) 
}

export async function signAndBroadcastTx(...msgs: Msg[]) {
    const signedTx = await _wallet.createAndSignTx({
        msgs
    });
    return await _client.tx.broadcast(signedTx);
}