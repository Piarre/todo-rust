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
    let description = input("Enter todo description: ");

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

    display_items(get_tasks(&db), true);

    let id = input("\nEnter the id to delete: ");

    if id.is_empty() || id == "q" {
        return;
    }

    if let Some(conn) = db.conn {
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
            .unwrap();
    }
}
