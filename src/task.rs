use std::io::{stdin, stdout, Write};

use crate::{
    db::{get_tasks, Database},
    utils::{self, input, wait_for_exit},
};

pub struct Task {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

pub fn display_items(tasks: Vec<Task>, display_id: bool) {
    for task in tasks {
        let status = if task.done { "✔" } else { "✘" };
        if display_id {
            println!("{} | [{}] | {}", task.id, status, task.description);
        } else {
            println!("[{}] | {}", status, task.description);
        }
    }
}

pub fn list(db: Database) {
    if let Some(conn) = db.conn {
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

        utils::clear();
        println!("--- | Rust | Todo App | ---");

        display_items(tasks.map(|t| t.unwrap()).collect(), false);

        print!("Press <q> to continue ");
        wait_for_exit();
        stdout().flush().unwrap();
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

pub fn delete(db: &Database) {
    utils::clear();

    display_items(get_tasks(&db), true);

    let id = input("\nEnter the id to delete: ");

    if id.is_empty() || id == "q" {
        return;
    }

    if let Some(conn) = &db.conn {
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
            .unwrap();
    }
}
