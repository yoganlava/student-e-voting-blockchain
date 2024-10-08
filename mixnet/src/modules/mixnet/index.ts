import { FastifyInstance } from "fastify";
import { MixnetNodeMsg, MixnetNodeMsgType } from "./types";
import {
    addNode,
    broadcastKeyCreateToNodes,
    getConnectedNodesCount,
    getNodeByID,
    getPublicKeys,
    removeNode,
    saveNodePublicKey,
} from "../nodes";
import { executeContractMessage, queryContract } from "../terra";
import { getDatabase } from "../db";
import { Worker } from "worker_threads";
import { v4 as uuidv4 } from "uuid";

const path = "/mixnet";

// list of call backs when sending encrypt message back to user;
// {
// id: fn
// }
let websocketCallback: {
    [key: string]: Function;
} = {};

//TODO: Make type
async function handleQueueMessage(message: any) {
    const nodeID = message.data.nodes_left.shift();

    if (!nodeID) {
        // TODO: handle
        return;
    }

    const connectedNode = getNodeByID(nodeID);

    if (!connectedNode) {
        // TODO: handle
        return;
    }

    console.log("Sending: ");
    console.log(message);
    console.log(`to ${nodeID}`);
    connectedNode.socket.send(JSON.stringify(message));
}

export default async function routes(
    fastify: FastifyInstance,
    options: Object
) {
    console.log(`Registered ${path}`);

    const queueWorker = new Worker("./src/workers/queueWorker.js");
    queueWorker.on("message", handleQueueMessage);

    fastify.post(path + "/encrypt", async (req, res) => {
        try {
            // cba to infer type
            const pollID = parseInt((req.body as any).pollID, 10);

            // // TODO: make type
            // const pollRes: any = await queryContract({
            //     Poll: {
            //         poll_id: pollID,
            //     },
            // });

            // if (pollRes.error) {
            //     res.send({
            //         error: "Invalid poll",
            //     });
            // }

            const db = await getDatabase();

            const keyOrder = await db.all(
                "SELECT node_id FROM key_order where poll_id = ? ORDER BY node_index ASC",
                pollID
            );

            //TODO send encrypt message

            const id = uuidv4();

            const finishPromise = new Promise((resolve) => {
                websocketCallback[id] = (vote: string) => {
                    console.log(`Executed callback: ${id}`);

                    delete websocketCallback[id];
                    resolve(vote);
                };
            });

            queueWorker.postMessage({
                type: "encrypt",
                data: {
                    poll_id: pollID,
                    vote: `${(req.body as any).vote}.${
                        (req.body as any).tracker
                    }`,
                    nodes_left: keyOrder.map((o) => o.node_id),
                },
                callback: id,
            });

            res.send(
                JSON.stringify({
                    encryptedVote: await finishPromise,
                })
            );
        } catch (e) {
            console.log(e);
            res.send({
                error: "Internal Server error",
            });
        }
    });

    // TODO make more secure
    fastify.post(path + "/notify/create", async (req, res) => {
        try {
            // // TODO: make type
            // const pollRes: any = await queryContract({
            //     Poll: {
            //         poll_id: pollID,
            //     },
            // });

            // if (pollRes.error) {
            //     res.send({
            //         error: "Invalid poll",
            //     });
            // }

            await broadcastKeyCreateToNodes((req.body as any).pollID);
        } catch (e) {
            console.log(e);
            res.send({
                error: "Internal Server error",
            });
        }
    });

    fastify.post(path + "/notify/finish", async (req, res) => {
        try {
            // cba to infer type
            const pollID = (req.body as any).pollID;

            // TODO: make type
            const pollRes: any = await queryContract({
                poll: {
                    poll_id: pollID,
                },
            });
            console.log(pollRes);

            if (pollRes.error) {
                res.send({
                    error: "Invalid poll",
                });
            }

            // TODO: make type
            const encryptedVotes: any = await queryContract({
                poll_votes: {
                    poll_id: pollID,
                },
            });
            console.log(encryptedVotes);

            const db = await getDatabase();

            const keyOrder = await db.all(
                "SELECT * FROM key_order where poll_id = ?",
                pollID
            );

            queueWorker.postMessage({
                type: "decrypt",
                data: {
                    poll_id: pollID,
                    votes: encryptedVotes,
                    nodes_left: keyOrder.reverse().map((o) => o.node_id),
                },
            });
        } catch (e) {
            console.log(e);
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

    fastify.get(path + "/keys", async (req, res) => {
        const pollID = (req.query as any).pollID;

        res.send({
            keys: await getPublicKeys(pollID)
        })
    });

    fastify.get(path + "/ws", { websocket: true }, async (conn, req) => {
        fastify.log.info("[MixNet] MixNet Node connected");

        conn.socket.on("message", async (message: Buffer) => {
            try {
                const msg: MixnetNodeMsg = JSON.parse(message.toString());
                console.log(msg);
                switch (msg.type) {
                    case MixnetNodeMsgType.KEY_RESPONSE:
                        await saveNodePublicKey(
                            msg.data.node_id,
                            msg.data.poll_id,
                            msg.data.key
                        );
                        break;
                    case MixnetNodeMsgType.REGISTER:
                        // if (!msg.data.id) {
                        //     conn.socket.send(
                        //         JSON.stringify({
                        //             error: "No id provided",
                        //         })
                        //     );
                        //     return;
                        // }
                        addNode(msg.data.id, conn);
                        fastify.log.info(
                            `[MixNet] MixNet Node registered - Node ID: ${msg.data.id}`
                        );
                        break;
                    // Result from encryption
                    case MixnetNodeMsgType.ENCRYPTION_RESULT:
                        console.log(`[MixNet] Got encryption result from node`);
                        // if (!msg.data.id) {
                        //     // conn.socket.send(
                        //     //     JSON.stringify({
                        //     //         error: "No id provided",
                        //     //     })
                        //     // );
                        //     return;
                        // }

                        if (!msg.data.nodes_left.length) {
                            // TODO: send back data
                            if (msg.callback)
                                websocketCallback[msg.callback](msg.data.vote);
                            return;
                        }

                        queueWorker.postMessage({
                            ...msg,
                            type: "encrypt",
                        });
                        break;
                    case MixnetNodeMsgType.DECRYPTION_RESULT:
                        console.log(`[MixNet] Got decryption result from node`);
                        // if (!msg.data.id) {
                        //     // conn.socket.send(
                        //     //     JSON.stringify({
                        //     //         error: "No id provided",
                        //     //     })
                        //     // );
                        //     return;
                        // }

                        if (!msg.data.nodes_left.length) {
                            // TODO: handle error
                            console.log("[MixNet] Pushing Unmixed Votes");
                            const res = await executeContractMessage({
                                push_unmixed_votes: {
                                    poll_id: msg.data.poll_id,
                                    votes: msg.data.votes,
                                },
                            });
                            console.log(res);
                            return;
                        }

                        queueWorker.postMessage(msg.data);
                        break;
                    default:
                        console.log(`Invalid msg type: ${msg.type}`);
                        console.log(msg);
                    // TODO: proper errors

                    // conn.socket.send(
                    //     JSON.stringify({
                    //         error: "Invalid msg type",
                    //     })
                    // );
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
            // TODO
            // removeNode(conn);
        });
    });

    console.log("Start queue");
}
