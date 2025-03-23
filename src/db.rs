use rusqlite::{Connection, Result};

pub fn init_database() -> Result<Connection> {
    let conn = Connection::open("earn_vault.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE,
            password TEXT,
            email TEXT,
            balance REAL
        )",
        [],
    )?;
    Ok(conn)
}