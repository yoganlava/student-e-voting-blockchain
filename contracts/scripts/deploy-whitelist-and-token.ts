import { LCDClient, LocalTerra, MnemonicKey } from "@terra-money/terra.js";
import { ADMIN_ADDRESS } from "./consts";
import { instantiateContract, queryContract, sleep, uploadContract } from "./utils";

const TOKEN_NAME = "Surrey Vote Token";
const TOKEN_SYMBOL = "SVT";
const TOKEN_DECIMALS = 6;
const TOKEN_DESCRIPTION = "Token used in Surrey to vote";
const TOKEN_PROJECT = "University Of Surrey";
const TOKEN_LOGO =
    "https://scontent-lhr8-1.xx.fbcdn.net/v/t39.30808-1/277673392_10159237483843241_4916018161413651394_n.jpg?stp=dst-jpg_p148x148&_nc_cat=108&ccb=1-7&_nc_sid=1eb0c7&_nc_ohc=4lQdC9sAMuUAX--sle8&_nc_ht=scontent-lhr8-1.xx&oh=00_AfBcXECvN5X7f1ZMDFr4B3tkOyGxkTy6QyojT6yjdjNvZw&oe=63982D81";

export default async (): Promise<string> => {

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

    const whitelistCodeID = await uploadContract(
        terra,
        wallet,
        "artifacts/cw1_whitelist.wasm"
    );

    console.log("Whitelist code ID:", whitelistCodeID);

    await sleep(1000);

    const whitelistAddress = await instantiateContract(
        terra,
        wallet,
        whitelistCodeID,
        {
            mutable: true,
            admins: [ADMIN_ADDRESS],
        }
    );

    console.log("Whitelist:", whitelistAddress);

    await sleep(1000);

    const tokenCodeId = await uploadContract(
        terra,
        wallet,
        "artifacts/cw20_base.wasm"
    );

    console.log("Token code ID:", tokenCodeId);

    await sleep(1000);

    const tokenAddress = await instantiateContract(terra, wallet, tokenCodeId, {
        name: TOKEN_NAME,
        symbol: TOKEN_SYMBOL,
        decimals: TOKEN_DECIMALS,
        initial_balances: [
            {
                address: wallet.key.accAddress,
                amount: "1000000000000",
            },
        ],
        mint: {
            minter: whitelistAddress,
        },
        // marketing: {
        //     marketing: ADMIN_ADDRESS,
        //     description: TOKEN_DESCRIPTION,
        //     project: TOKEN_PROJECT,
        //     logo: {
        //         url: TOKEN_LOGO
        //     }
        // }
    });

    console.log("Token:", tokenAddress);
    return tokenAddress;
};
