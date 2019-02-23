use std::env::var;
use std::fs::{metadata, read_to_string};

fn main() {
    let default_path = var("HOME").unwrap() + "/.todo_db";
    let path = var("TODO_DB").unwrap_or(default_path);
    if metadata(&path).is_ok() {
        let contents = read_to_string(&path).unwrap();
        let lines: Vec<&str> = contents
            .trim()
            .split("\n")
            .filter(|x| x.trim().len() > 0)
            .collect();
        for line in lines {
            println!("• {}", line);
        }
    }
}
