import { FastifyInstance } from "fastify";
import { MixnetNodeMsg, MixnetNodeMsgType } from "./types";
import {
    addNode,
    broadcastKeyCreateToNodes,
    getConnectedNodesCount,
    getNodeByID,
    removeNode,
} from "../nodes";
import { executeContractMessage, queryContract } from "../terra";
import { getDatabase } from "../db";
import { Worker } from "worker_threads";

const path = "/mixnet";

//TODO: Make type
async function handleQueueMessage(message: any) {
    const nodeID = message.nodes_left.shift();

    if (!nodeID) {
        // TODO: handle
        return;
    }

    const connectedNode = getNodeByID(nodeID);

    if (!connectedNode) {
        // TODO: handle
        return;
    }

    connectedNode.socket.send(JSON.stringify(message));
}

export default async function routes(
    fastify: FastifyInstance,
    options: Object
) {
    console.log(`Registered ${path}`);

    const queueWorker = new Worker("./src/workers/queueWorker.ts");
    queueWorker.on("message", handleQueueMessage);

    fastify.post(path + "/notify/create/:pollID", async (req, res) => {
        try {
            // cba to infer type
            const pollID = parseInt((req.params as any).pollID);

            // TODO: make type
            const pollRes: any = await queryContract({
                Poll: {
                    poll_id: pollID,
                },
            });

            if (pollRes.error) {
                res.send({
                    error: "Invalid poll",
                });
            }

            await broadcastKeyCreateToNodes(await getDatabase(), pollID);
        } catch (e) {
            res.send({
                error: "Internal Server error",
            });
        }
    });

    fastify.post(path + "/notify/finish/:pollID", async (req, res) => {
        try {
            // cba to infer type
            const pollID = parseInt((req.params as any).pollID);

            // TODO: make type
            const pollRes: any = await queryContract({
                Poll: {
                    poll_id: pollID,
                },
            });

            if (pollRes.error) {
                res.send({
                    error: "Invalid poll",
                });
            }

            // TODO: make type
            const votesRes: any = await queryContract({
                EncryptedVotes: {
                    pollID,
                },
            });

            const db = await getDatabase();

            const keyOrder = await db.all(
                "SELECT * FROM key_order where poll_id = ?",
                pollID
            );

            queueWorker.postMessage(
                {
                    type: "decrypt",
                    data: {
                        poll_id: pollID,
                        votes: votesRes.encrypted_votes,
                        nodes_left: keyOrder.map((o) => o.node_id),
                    }
                }
            );
        } catch (e) {
            res.send({
                error: "Internal Server error",
            });
        }
    });

    fastify.get(path + "/nodes", async (req, res) => {
        res.send({
            nodes: getConnectedNodesCount(),
        });
    });

    fastify.get(path + "/ws", { websocket: true }, async (conn, req) => {
        fastify.log.info("[MixNet] MixNet Node connected");

        conn.socket.on("message", async (message: Buffer) => {
            try {
                const msg: MixnetNodeMsg = JSON.parse(message.toString());
                switch (msg.type) {
                    case MixnetNodeMsgType.REGISTER:
                        if (!msg.data.id) {
                            conn.socket.send(
                                JSON.stringify({
                                    error: "No id provided",
                                })
                            );
                            return;
                        }
                        // addNode(conn);
                        fastify.log.info(
                            `[MixNet] MixNet Node registered - Node ID: ${msg.data.id}`
                        );
                        conn.socket.send(
                            JSON.stringify({
                                message: "Node registered",
                            })
                        );
                        break;
                    // Result from encryption
                    case MixnetNodeMsgType.ENCRYPTION_RESULT:
                        if (!msg.data.id) {
                            conn.socket.send(
                                JSON.stringify({
                                    error: "No id provided",
                                })
                            );
                            return;
                        }

                        if (!msg.data.nodes_left.length) {
                            // TODO: send back data
                        }

                        queueWorker.postMessage(msg);
                        break;
                    case MixnetNodeMsgType.DECRYPTION_RESULT:
                        if (!msg.data.id) {
                            conn.socket.send(
                                JSON.stringify({
                                    error: "No id provided",
                                })
                            );
                            return;
                        }

                        if (!msg.data.nodes_left.length) {
                            // TODO: handle error
                            await executeContractMessage({
                                ExecutePushUnMixedVotes: {
                                    poll_id: msg.data.poll_id,
                                    votes: msg.data.votes
                                }
                            });
                            return;
                        }

                        queueWorker.postMessage(msg.data);
                        break;
                    default:
                        conn.socket.send(
                            JSON.stringify({
                                error: "Invalid msg type",
                            })
                        );
                }
            } catch (e) {
                console.log(e);
                conn.socket.send(
                    JSON.stringify({
                        error: "Something went wrong",
                    })
                );
            }
        });

        conn.socket.on("disconnect", () => {
            fastify.log.info("[MixNet] MixNet Node disconnected");
            // removeNode(conn);
        });
    });

    console.log("Start queue");
}
