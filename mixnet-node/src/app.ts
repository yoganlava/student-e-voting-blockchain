import WebSocket from "ws";
import dotenv from "dotenv";
import { v4 as uuidv4 } from "uuid";
import { getDatabase } from "./db";
import { MixnetMessage, MixnetMessageType } from "./types";
import { Database } from "sqlite";
import { PrivateKey, decrypt } from "eciesjs";

dotenv.config();

// TODO: Make proper logging?
// TODO: split into files if bothered

const ws = new WebSocket(process.env.MIXNET_ADDR as string);

async function getAndCommitUUID(db: Database) {
    const uuid = uuidv4();

    await db.run("INSERT INTO id (id) VALUES (?)", uuid);

    return uuid;
}

async function getAndCommitKey(db: Database, pollID: number) {
    let existingKeypair;
    if ((existingKeypair = await getKeypair(db, pollID)))
        return existingKeypair.public_key;

    const privateKey = new PrivateKey();
    const publicKey = privateKey.publicKey.toHex();

    await db.run(
        "INSERT INTO keys (poll_id, public_key, private_key) VALUES (?, ?, ?)",
        pollID,
        publicKey,
        privateKey.toHex()
    );

    return publicKey;
}

async function getKeypair(db: Database, pollID: number) {
    return await db.get("SELECT * FROM keys WHERE poll_id = ?", pollID);
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
            case MixnetMessageType.CREATE_KEY:
                console.log(
                    `[MixNet Node] Create key message recieved - Poll ID: ${msg.data.poll_id}`
                );
                ws.send(await getAndCommitKey(db, msg.data.poll_id));
                break;
            case MixnetMessageType.DECRYPT: {
                console.log(
                    `[MixNet Node] Decrypt message recieved - Poll ID: ${msg.data.poll_id}`
                );
                const keypair = await getKeypair(db, msg.data.poll_id);

                if (!keypair) {
                    ws.send(
                        JSON.stringify({
                            error: "Invalid poll id",
                        })
                    );
                    break;
                }

                ws.send(JSON.stringify(
                    {
                        decrypted_message: decrypt(keypair.private_key, msg.data.vote)
                    }
                ));

                break;
            }
            default:
                ws.send(
                    JSON.stringify({
                        error: "Invalid msg type",
                    })
                );
        }
    } catch (e) {
        console.log(e);
    }
});
