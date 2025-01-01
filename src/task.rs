use std::io::stdin;

use crate::utils::{self, sleep};

pub struct Task {
    pub title: String,
    pub done: bool,
}

pub fn get_all(tasks: &[Task]) {
    utils::clear();

    if tasks.is_empty() {
        println!("No task found.");
        utils::sleep(1);
    } else {
        for (_, task) in tasks.iter().enumerate() {
            let status = if task.done { "✔" } else { "✘" };
            println!("[{}] | {}", status, task.title)
        }

        println!("Presse q to exit")
    }
}

pub fn add(tasks: &mut Vec<Task>) {
    utils::clear();
    print!("Enter todo: ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();

    tasks.push(Task { title, done: false });
}
