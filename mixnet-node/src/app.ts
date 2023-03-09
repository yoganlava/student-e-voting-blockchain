import WebSocket from "ws";
import dotenv from "dotenv";
import { v4 as uuidv4 } from "uuid";
import { getDatabase } from "./db";
import { MixnetMessage, MixnetMessageType } from "./types";
import { Database } from "sqlite";
import { PrivateKey, decrypt, encrypt } from "eciesjs";

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

    let uuid = idRes?.id ?? (await getAndCommitUUID(db));

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
                const publicKey = await getAndCommitKey(db, msg.data.poll_id);

                let nodeID = await db.get("SELECT id FROM ID");

                ws.send(
                    JSON.stringify({
                        type: "key_response",
                        data: {
                            key: publicKey,
                            poll_id: msg.data.poll_id,
                            node_id: nodeID,
                        },
                    })
                );
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

                console.log(msg.data.votes[0]);

                ws.send(
                    JSON.stringify({
                        type: "decryption_result",
                        data: {
                            ...msg.data,
                            // TODO: make proper function
                            votes: msg.data.votes.map((vote: any) => {
                                const decryptedVote = {
                                    ...vote,
                                    // TODO: If fails to decrypt, turn vote.malformed true
                                    encrypted_vote: [
                                        ...decrypt(
                                            keypair.private_key,
                                            Buffer.from(vote.encrypted_vote)
                                        ),
                                    ],
                                };

                                if (!msg.data.nodes_left.length) {
                                    decryptedVote.decrypted_vote = Buffer.from(
                                        decryptedVote.encrypted_vote
                                    ).toString();
                                }
                                return decryptedVote;
                            }),
                        },
                    })
                );

                break;
            }
            case MixnetMessageType.ENCRYPT: {
                console.log(
                    `[MixNet Node] Encrypt message recieved - Poll ID: ${msg.data.poll_id}`
                );
                const keypair = await getKeypair(db, msg.data.poll_id);

                if (!keypair) {
                    // TODO: proper errors
                    // ws.send(
                    //     JSON.stringify({
                    //         error: "Invalid poll id",
                    //     })
                    // );
                    break;
                }

                ws.send(
                    JSON.stringify({
                        // decrypted_message: encrypt(keypair.public_key, msg.data.vote)
                        // TODO: make type
                        type: "encryption_result",
                        data: {
                            ...msg.data,
                            vote: encrypt(keypair.public_key, msg.data.vote),
                        },
                        callback: msg.callback,
                    })
                );

                break;
            }
            default:
                console.log(`Invalid msg type: ${msg.type}`);
                console.log(msg);
            // ws.send(
            //     JSON.stringify({
            //         error: `Invalid msg type: ${msg.type}`,
            //     })
            // );
        }
    } catch (e) {
        console.log(e);
    }
});
