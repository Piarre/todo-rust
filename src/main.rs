mod db;
mod task;
mod utils;

use db::Database;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let sqlite = Database::new();

        utils::clear();
        println!("--- | Rust | Todo App | ---");
        println!("(1) | Get all todos");
        println!("(2) | Add a todo");
        println!("(3) | Delete a todo");
        println!("(5) | Quit");

        let mut input = String::new();
        print!("?: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => task::list(sqlite),
            "2" => task::add(sqlite),
            "3" => task::delete(&sqlite),
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
