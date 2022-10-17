import { JsonRpc } from "node-jsonrpc-client";

export default class Blockchain {
    _client: JsonRpc;

    public constructor(connectionURL: string) {
        this._client = new JsonRpc(connectionURL);
    }

    public async getLatestBlock() {
        return await this._client.call("get_latest_block", {});
    }
}