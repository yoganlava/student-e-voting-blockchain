import WebSocket from "ws";
import dotenv from "dotenv";
import { v4 as uuidv4 } from "uuid";
import { getDatabase } from "./db";
import { MixnetMessage, MixnetMessageType } from "./types";
import { Database } from "sqlite";

dotenv.config();

// TODO: Make proper logging?

const ws = new WebSocket(process.env.MIXNET_ADDR as string);

async function getAndCommitUUID(db: Database) {
    const uuid = uuidv4();

    await db.run("INSERT INTO ID (id) VALUES (?)", uuid);

    return uuid;
}

ws.on("open", async () => {
    const db = await getDatabase();

    let idRes = await db.get("SELECT id FROM ID");

    let uuid = idRes?.id ?? getAndCommitUUID(db);

    ws.send(
        JSON.stringify({
            type: "register",
            data: {
                id: uuid,
            },
        })
    );
});

ws.on("message", async (data) => {
    try {
        const msg: MixnetMessage = JSON.parse(data.toString());
        const db = await getDatabase();
        switch (msg.type) {
            case MixnetMessageType.KEY:
                console.log("[MixNet Node] Key recieved")
                await db.run("INSERT INTO KEYS (poll_id, key) VALUES (?, ?)", msg.data.poll_id, msg.data.key)
                break;
            default:
                ws.send(
                    JSON.stringify({
                        error: "Invalid msg type"
                    })
                );
        }
    } catch (e) {
        console.log(e);
    }
});
