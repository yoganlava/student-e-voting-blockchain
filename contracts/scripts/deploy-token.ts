import { LocalTerra } from "@terra-money/terra.js";

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
})();
