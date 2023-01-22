import Fastify from "fastify";
import websocket from "@fastify/websocket";
import mixnet from "./modules/mixnet";

const fastify = Fastify({
    logger: {
        transport: {
            target: "pino-pretty",
        },
    },
});

fastify.get("/", async (req, res) => {
    res.send("Mix net running...");
});

fastify.register(websocket);
fastify.register(mixnet);

fastify.listen({
    port: 3000,
});
