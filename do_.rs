use std::env::{args, var};
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let mut args: Vec<String> = args().collect();
    let default_path = var("HOME").unwrap() + "/.todo_db";
    let path = var("TODO_DB").unwrap_or(default_path);
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let append = args.len() > 1;
    if append {
        args.remove(0);
    }
    let joined_args = args.join(" ");
    let todo = joined_args.trim();
    let lines: Vec<&str> = contents
        .trim()
        .split("\n")
        .filter(|line| line.trim().len() > 0)
        .collect();
    for line in &lines {
        println!("\x1B[37m• {}\x1B[0m", line);
    }
    if append {
        if !lines.contains(&todo) {
            println!("\x1B[1;32m+ {}\x1B[0m", todo);
            let prefix = if contents.ends_with("\n") { "" } else { "\n" };
            writeln!(file, "{}", prefix.to_owned() + &todo).unwrap();
        }
    }
}
