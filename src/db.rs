use rusqlite::{params, Connection, Result};

pub fn init_database() -> Result<Connection> {
    let conn = Connection::open("earn_vault.db")?;

    // Create Users Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            balance REAL DEFAULT 0.0
        )",
        [],
    )?;

    // Create Transactions Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            transaction_type TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    Ok(conn)
}

// Function to create a new user
pub fn create_user(conn: &Connection, username: &str, password: &str, email: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, password, email) VALUES (?, ?, ?)",
        params![username, password, email],
    )?;
    Ok(())
}

// Function to get a user balance
pub fn get_user_balance(conn: &Connection, username: &str) -> Result<f64> {
    let mut stmt = conn.prepare("SELECT balance FROM users WHERE username = ?")?;
    let balance: f64 = stmt.query_row(params![username], |row| row.get(0))?;
    Ok(balance)
}

// Function to update user balance
pub fn update_balance(conn: &Connection, username: &str, amount: f64) -> Result<()> {
    conn.execute(
        "UPDATE users SET balance = balance + ? WHERE username = ?",
        params![amount, username],
    )?;
    Ok(())
}

// Function to record a transaction
pub fn record_transaction(conn: &Connection, user_id: i32, amount: f64, transaction_type: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (user_id, amount, transaction_type) VALUES (?, ?, ?)",
        params![user_id, amount, transaction_type],
    )?;
    Ok(())
}