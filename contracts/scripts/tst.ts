import { LocalTerra, MsgSend } from "@terra-money/terra.js";
import { ADMIN_ADDRESS } from "./consts";
import { makeTransaction } from "./utils";

(async () => {
    let terra = new LocalTerra();
    let wallet = terra.wallets.test1;

    const sendMsg = new MsgSend(wallet.key.accAddress, ADMIN_ADDRESS, {
        uluna: "100000",
    });

    console.log(await makeTransaction(terra, wallet, sendMsg));
})();
