import { LCDClient, LocalTerra, MnemonicKey } from "@terra-money/terra.js";
import { instantiateContract, sleep, uploadContract } from "./utils";
import { MIXNET_ADDRESS } from "./consts";

export default async (tokenAddress: string) => {
    const terra = new LCDClient({
        URL: "https://pisco-lcd.terra.dev",
        chainID: "pisco-1",
    });

    let wallet = terra.wallet(
        new MnemonicKey({
            mnemonic:
                "penalty draw glove enforce dog olive wild bean edit sick fantasy goddess payment welcome kidney fish group type era foot ticket video then woman",
        })
    );

    // const terra = new LocalTerra();
    // const wallet = terra.wallets.test1;

    const evotingCodeID = await uploadContract(
        terra,
        wallet,
        "artifacts/e_voting.wasm"
    );

    console.log("E-voting code ID:", evotingCodeID);

    await sleep(1000);

    const contractAddress = await instantiateContract(
        terra,
        wallet,
        evotingCodeID,
        {
            voting_token_addr: tokenAddress,
            mixnet_addr: MIXNET_ADDRESS
        }
    );

    console.log("Contract addr:", contractAddress);
};
