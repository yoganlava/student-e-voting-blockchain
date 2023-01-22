import { SocketStream } from "@fastify/websocket";

let _connectedNodes: Set<SocketStream> = new Set<SocketStream>();

export function addNode(conn: SocketStream) {
    _connectedNodes.add(conn);
}

export function removeNode(conn: SocketStream) {
    _connectedNodes.delete(conn);
}

export function getConnectedNodes() {
    return [..._connectedNodes];
}
