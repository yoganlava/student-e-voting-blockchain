import {
    isTxError,
    MsgInstantiateContract,
    MsgStoreCode,
} from "@terra-money/terra.js";

import fs from "fs";

export async function createSignedTransaction(client, wallet, msg) {
    // const { account_number, sequence } = await wallet.accountNumberAndSequence();
    return await wallet.createAndSignTx({
        msgs: [msg],
    });
    // const tx = await wallet.createTx({
    //     msgs: [msg],
    // });
    // const { account_number, sequence } =
    //     await wallet.accountNumberAndSequence();
    // console.log("Signing tx");
    // return await wallet.key.signTx(tx, {
    //     accountNumber: account_number,
    //     sequence: sequence,
    //     chainID: client.config.chainID,
    //     signMode: 1,
    // });
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
    const signedTransaction = await createSignedTransaction(
        client,
        wallet,
        msg
    );
    console.log("Signed tx");
    const result = await broadcastTransaction(client, signedTransaction);
    console.log("Broadcasting TX");
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
        console.log("Created msg");
        let result: any = await makeTransaction(client, wallet, uploadMsg);
        return Number(result.logs[0].eventsByType.store_code.code_id[0]);
    } catch (e) {
        console.error(`${filepath} Upload Error:`);
        console.error(e);
    }
}

export async function queryContract(terra, contractAddress, query) {
    try {
        return await terra.wasm.contractQuery(contractAddress, query);
    } catch (e) {
        console.error(`Query Error:`);
        console.error(e);
    }

    
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
