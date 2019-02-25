mod shared;

use shared::{get_todos, init_path};
use std::fs::{metadata, read_to_string, write};

fn main() {
    let path = init_path();
    if metadata(&path).is_ok() {
        let todos = get_todos();
        let contents = read_to_string(&path).unwrap();
        let lines: Vec<String> = contents
            .trim()
            .split("\n")
            .map(|line| line.to_string())
            .filter(|line| {
                if line.trim().len() == 0 {
                    false
                } else if todos.contains(line) {
                    println!("\x1B[31m- {}\x1B[0m", line);
                    false
                } else {
                    println!("• {}", line);
                    true
                }
            })
            .collect();
        let new_contents = lines.join("\n") + "\n";
        write(path, new_contents.as_bytes()).unwrap();
    }
}
