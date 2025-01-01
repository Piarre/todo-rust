use std::io::stdin;

use crate::{
    db::Database,
    utils::{self, sleep, wait_for_exit},
};

pub struct Task {
    pub description: String,
    pub done: bool,
}

pub fn get_all_tasks(db: Database) {
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
            let status = if task.done { "Done" } else { "Not done" };
            println!("{} | {}", task.description, status);
        }

        wait_for_exit();
    }
}

pub fn add(tasks: &mut Vec<Task>) {
    utils::clear();
    print!("Enter todo: ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();

    tasks.push(Task {
        description: title,
        done: false,
    });
}
