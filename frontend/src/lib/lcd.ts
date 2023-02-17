import { createLCDClient } from "@terra-money/wallet-controller";

let _instance = createLCDClient({
    network: {
        name: "testnet",
        chainID: "bombay-12",
        lcd: "https://bombay-lcd.terra.dev",
        walletconnectID: 0
    }
});

export default function getLCDClient() {
    return _instance;
}