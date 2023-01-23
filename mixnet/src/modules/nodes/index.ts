import { SocketStream } from "@fastify/websocket";

let _connectedNodes: Set<SocketStream>;

export function initialiseNodes(): void {
    if (!_connectedNodes) _connectedNodes = new Set<SocketStream>();
}

export function addNode(conn: SocketStream): void {
    _connectedNodes.add(conn);
}

export function removeNode(conn: SocketStream): void {
    _connectedNodes.delete(conn);
}

export function getConnectedNodesCount(): number {
    return _connectedNodes.size;
}

export function broadcastToNodes(message: Object): void {
    _connectedNodes.forEach((conn) => {
        conn.socket.send(JSON.stringify(message));
    });
}
