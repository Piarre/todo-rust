use rusqlite::{Connection, Params};

pub struct Database {
    pub conn: Option<Connection>,
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("todo.db").expect("Failed to open database.");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL
            )",
            [],
        )
        .unwrap();

        Database { conn: Some(conn) }
    }

    pub fn close(&mut self) {
        if let Some(conn) = self.conn.take() {
            conn.close().unwrap();
        }
    }
}
