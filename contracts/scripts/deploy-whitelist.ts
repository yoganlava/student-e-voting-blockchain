import { LocalTerra } from "@terra-money/terra.js";
import { ADMIN_ADDRESS } from "./consts";
import { instantiateContract, queryContract, uploadContract } from "./utils";

const TOKEN_NAME = "Surrey Vote Token";
const TOKEN_SYMBOL = "SVT";
const TOKEN_DECIMALS = 6;
const TOKEN_DESCRIPTION = "Token used in Surrey to vote";
const TOKEN_PROJECT = "University Of Surrey";
const TOKEN_LOGO =
    "https://scontent-lhr8-1.xx.fbcdn.net/v/t39.30808-1/277673392_10159237483843241_4916018161413651394_n.jpg?stp=dst-jpg_p148x148&_nc_cat=108&ccb=1-7&_nc_sid=1eb0c7&_nc_ohc=4lQdC9sAMuUAX--sle8&_nc_ht=scontent-lhr8-1.xx&oh=00_AfBcXECvN5X7f1ZMDFr4B3tkOyGxkTy6QyojT6yjdjNvZw&oe=63982D81";

(async () => {
    let terra = new LocalTerra();
    let wallet = terra.wallets.test1;

    const whitelistCodeId = await uploadContract(
        terra,
        wallet,
        "artifacts/cw1_whitelist.wasm"
    );

    console.log("Whitelist code ID:", whitelistCodeId);

    const whitelistAddress = await instantiateContract(
        terra,
        wallet,
        whitelistCodeId,
        {
            mutable: true,
            admins: [ADMIN_ADDRESS],
        }
    );

    console.log("Whitelist:", whitelistAddress);
    console.log(await terra.wasm.contractInfo(whitelistAddress));

    const tokenCodeId = await uploadContract(
        terra,
        wallet,
        "artifacts/cw20_base.wasm"
    );

    console.log("Token code ID:", tokenCodeId);

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
    console.log(
        JSON.stringify(await terra.wasm.contractInfo(tokenAddress), null, 4)
    );

    console.log(await queryContract(terra, tokenAddress, { token_info: {} }));
})();
