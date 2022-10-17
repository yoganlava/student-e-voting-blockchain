import Blockchain from "./blockchain";

let _blockchain;

export default function getBlockchain() {
    if (_blockchain) return _blockchain;
    _blockchain = new Blockchain(process.env.BLOCKCHAIN_CONNECTION_URL);
    return _blockchain;
}