mod task;
mod utils;

use std::io::{stdin, stdout, Write};
use task::Task;

fn main() {
    let mut tasks: Vec<Task> = Vec::<Task>::new();

    utils::clear();

    loop {
        utils::clear();
        println!("--- | Rust | Todo App | ---");
        println!("(1) | Get all todos");
        println!("(2) | Add todo");
        println!("(5) | Quit");

        let mut input = String::new();
        print!("?: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "5" => {
                utils::clear();
                break;
            }
            _ => {
                utils::clear();
                println!("Wrong choice, try again...");
                utils::sleep(1);
            }
        }
    }
}
