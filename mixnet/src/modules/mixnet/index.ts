import { FastifyInstance } from "fastify";
import { MixnetNodeMsg, MixnetNodeMsgType } from "./types";
import { addNode, getConnectedNodes, removeNode } from "../nodes";

const path = "/mixnet";

export default async function routes(
    fastify: FastifyInstance,
    options: Object
) {
    console.log(`Registered ${path}`);
    fastify.post(path + "/notify", async (req, res) => {});

    fastify.get(path + "/nodes", async (req, res) => {
        res.send({
            nodes: getConnectedNodes().length,
        });
    });

    fastify.get(path + "/ws", { websocket: true }, async (conn, req) => {
        fastify.log.info("[MixNet] MixNet Node connected");

        conn.socket.on("message", (message: Buffer) => {
            try {
                const msg: MixnetNodeMsg = JSON.parse(message.toString());
                switch (msg.type) {
                    case MixnetNodeMsgType.REGISTER:
                        fastify.log.info("[MixNet] MixNet Node registered");
                        addNode(conn);
                        conn.socket.send(
                            JSON.stringify({
                                message: "Node registered",
                            })
                        );
                        break;
                    // Result from encryption
                    case MixnetNodeMsgType.RESULT:
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
            removeNode(conn);
        });
    });
}
