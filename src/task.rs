use std::io::stdin;

use crate::{
    db::Database,
    utils::{self, wait_for_exit},
};

pub struct Task {
    pub description: String,
    pub done: bool,
}

pub fn list(db: Database) {
    if let Some(conn) = db.conn {
        let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();

        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    description: row.get(1)?,
                    done: row.get(2)?,
                })
            })
            .unwrap();

        utils::clear();
        println!("--- | Rust | Todo App | ---");

        for task in tasks {
            let task = task.unwrap();
            let status = if task.done { "✔" } else { "✘" };
            println!("[{}] | {}", status, task.description);
        }

        wait_for_exit();
    }
}

pub fn add(db: Database) {
    utils::clear();
    print!("Enter todo: ");
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();

    if let Some(conn) = db.conn {
        conn.execute(
            "INSERT INTO tasks (description, done) VALUES  (?1, ?2)",
            (&description.trim(), false),
        )
        .unwrap();
    }
}

pub fn delete(db: Database) {
    utils::clear();
    println!("Enter todo to delete: ");
    let mut id = String::new();
    stdin().read_line(&mut id).unwrap();

    println!("{}", id);

    if let Some(conn) = db.conn {
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
            .unwrap();
    }
}
