import { SocketStream } from "@fastify/websocket";
import { Database } from "sqlite";

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

export function getNodeByID(id: string) {
    return _connectedNodes.get(id);
}
// Fisher-Yates
// https://stackoverflow.com/questions/2450954/how-to-randomize-shuffle-a-javascript-array
function shuffle(array: Array<any>): Array<any> {
    let shuffledArray = [...array];

    for (
        let currentIndex = array.length - 1;
        currentIndex > 0;
        currentIndex--
    ) {
        const randomIndex = Math.floor(Math.random() * (currentIndex + 1));
        [array[currentIndex], array[randomIndex]] = [
            array[randomIndex],
            array[currentIndex],
        ];
    }

    return shuffledArray;
}

export async function broadcastKeyCreateToNodes(
    db: Database,
    pollID: number
): Promise<void> {
    /*
    TODO: Randomly select connected nodes
    TODO: Asks keys to be generated
    TODO: Save keys in key_order table so when it is time for encryption/decryption...
    TODO: ...we know in what order to encrypt/decrypt
    */

    const shuffledNodes: Array<[string, SocketStream]> = shuffle(
        Array.from(_connectedNodes.entries())
    );

    for (let i = 0; i < shuffledNodes.length; i++) {
        shuffledNodes[i][1].socket.send(
            JSON.stringify({
                type: "create_key",
                poll_id: pollID,
            })
        );

        await db.run(
            "INSERT INTO key_order (poll_id, node_id, index)",
            pollID,
            shuffledNodes[i][0],
            i
        );
    }
}
