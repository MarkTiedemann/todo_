use std::env::{args, var};
use std::fs::{metadata, read_to_string, write};

fn main() {
    let mut args: Vec<String> = args().collect();
    let default_path = var("HOME").unwrap() + "/.todo_db";
    let path = var("TODO_DB").unwrap_or(default_path);
    if metadata(&path).is_ok() {
        let contents = read_to_string(&path).unwrap();
        let remove = args.len() > 1;
        if remove {
            args.remove(0);
        }
        let joined_args = args.join(" ");
        let todo = joined_args.trim();
        let lines: Vec<&str> = contents
            .trim()
            .split("\n")
            .filter(|line| {
                if line.trim().len() == 0 {
                    false
                } else if remove && line == &todo {
                    println!("\x1B[1;31m- {}\x1B[0m", todo);
                    false
                } else {
                    println!("• {}", line);
                    true
                }
            })
            .collect();
        if remove {
            let new_contents = lines.join("\n") + "\n";
            write(path, new_contents.as_bytes()).unwrap();
        }
    }
}
