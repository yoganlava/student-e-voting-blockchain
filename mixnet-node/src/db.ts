import sqlite3 from "sqlite3";
import { Database, open } from 'sqlite'

const migration = `
CREATE TABLE IF NOT EXISTS ID (id char);

CREATE TABLE IF NOT EXISTS KEYS (poll_id integer, key varchar);
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



