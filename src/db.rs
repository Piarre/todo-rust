use rusqlite::Connection;

use crate::task::Task;

pub struct Database {
    pub conn: Option<Connection>,
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("todo.db").unwrap();

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
}

pub fn get_tasks(db: &Database) -> Vec<Task> {
    if let Some(conn) = &db.conn {
        let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();

        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    done: row.get(2)?,
                })
            })
            .unwrap();

        tasks.map(|t| t.unwrap()).collect()
    } else {
        vec![]
    }
}
