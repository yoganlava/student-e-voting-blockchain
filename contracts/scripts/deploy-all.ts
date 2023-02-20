import deployContract from "./deploy-contract";
import deployWhitelistAndToken from "./deploy-whitelist-and-token"

(async () => {
    // const tokenAddress = await deployWhitelistAndToken();
    const tokenAddress = "terra1rmrpwkyf9vuat9ekfd9qwh5hlhdutg2rw6mcutvuasenq68y4mxs578un7";
    await deployContract(tokenAddress);
})()