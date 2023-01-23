import { SocketStream } from "@fastify/websocket";

let _connectedNodes: Map<string, SocketStream>;

export function initialiseNodes(): void {
    if (!_connectedNodes) _connectedNodes = new Map<string, SocketStream>();
}

export function addNode(connID: string, conn: SocketStream): void {
    _connectedNodes.set(connID, conn);
}

export function removeNode(connID: string): void {
    _connectedNodes.delete(connID);
}

export function getConnectedNodesCount(): number {
    return _connectedNodes.size;
}

export function broadcastKeyToNodes(/* TODO: args */): void {
    /*
    TODO: Randomly select connected nodes
    TODO: Send key to each
    TODO: Save keys in key_order table so when it is time for encryption/decryption...
    TODO: ...we know in what order to encrypt/decrypt
    */
}
