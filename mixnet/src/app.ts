import Fastify from "fastify";
import cors from "@fastify/cors";
import websocket from "@fastify/websocket";
import mixnet from "./modules/mixnet";
import dotenv from "dotenv";
import { initialiseClientAndWallet } from "./modules/terra";
import { initialiseNodes } from "./modules/nodes";
dotenv.config();
initialiseClientAndWallet();
initialiseNodes();

// TODO: Write unit tests if possible

const fastify = Fastify({
    logger: {
        transport: {
            target: "pino-pretty",
        },
    },
});
fastify.register(cors, {
    origin: "*",
});

fastify.get("/", async (req, res) => {
    res.send("Mix net running...");
});

fastify.register(websocket);
fastify.register(mixnet);

fastify.listen({
    port: 3000,
});
