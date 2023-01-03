import {
    isTxError,
    MsgInstantiateContract,
    MsgStoreCode,
} from "@terra-money/terra.js";

import fs from "fs";

export async function createSignedTransaction(wallet, msg) {
    return await wallet.createAndSignTx({
        msgs: [msg],
    });
}

export async function sleep(timeout: number) {
    await new Promise((resolve) => setTimeout(resolve, timeout));
}

export async function broadcastTransaction(client, signedTransaction) {
    const result = await client.tx.broadcast(signedTransaction);
    await sleep(1000);
    return result;
}

export async function makeTransaction(client, wallet, msg) {
    const signedTransaction = await createSignedTransaction(wallet, msg);
    const result = await broadcastTransaction(client, signedTransaction);
    if (isTxError(result)) {
        console.error("ERROR: ");
        console.error(result);
        process.exit(1);
    }
    return result;
}

export async function uploadContract(client, wallet, filepath) {
    try {
        const contract = fs.readFileSync(filepath, "base64");
        const uploadMsg = new MsgStoreCode(wallet.key.accAddress, contract);
        let result: any = await makeTransaction(client, wallet, uploadMsg);
        return Number(result.logs[0].eventsByType.store_code.code_id[0]);
    } catch (e) {
        console.error("Upload Error:");
        console.error(e);
    }
}

export async function queryContract(terra, contractAddress, query) {
    return await terra.wasm.contractQuery(contractAddress, query);
}

export async function instantiateContract(
    client,
    wallet,
    codeId,
    msg,
    opts = {} as any
) {
    try {
        let admin = opts.admin;
        if (admin == undefined) {
            admin = wallet.key.accAddress;
        }
        const instantiateMsg = new MsgInstantiateContract(
            wallet.key.accAddress,
            admin,
            codeId,
            msg,
            {},
            "Contract"
        );
        let result: any = await makeTransaction(client, wallet, instantiateMsg);
        const attributes = result.logs[0].events[0].attributes;
        return attributes[0].value;
    } catch (e) {
        console.error("Instantiate Error:");
        console.error(e);
    }
}
