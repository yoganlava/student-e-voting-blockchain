import sqlite3 from "sqlite3";
import { Database, open } from 'sqlite'

// key_order stores which nodes to send votes to and in which order
// reverse the ordering to get the decryption order
const migration = `
CREATE TABLE IF NOT EXISTS key_order (poll_id int, node_id varchar, index int);
`

let _db: Database;

export async function getDatabase(){
    if (_db) return _db;

    _db = await open({
        filename: "db/db.sqlite",
        driver: sqlite3.Database
    });

    await _db.exec(migration);
    return _db;
}



