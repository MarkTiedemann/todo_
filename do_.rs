use std::env::{args, var};
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let home = var("HOME").unwrap();
    let default_path = home.clone() + "/.todo_list";
    let path = var("TODO_LIST").unwrap_or(default_path);
    if var("TODO_PRINT_PATH").is_ok() {
        println!("\x1B[35m{}\x1B[0m", path.replace(&home, "~"));
    }

    let mut args: Vec<String> = args().collect();
    let append = args.len() > 1;
    if append {
        args.remove(0);
    }
    let joined_args = args.join(" ");
    let todo = joined_args.trim();

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents
        .trim()
        .split("\n")
        .filter(|line| line.trim().len() > 0)
        .collect();

    for line in &lines {
        println!("• {}", line);
    }

    if append {
        if !lines.contains(&todo) {
            println!("\x1B[32m+ {}\x1B[0m", todo);
            let prefix = if contents.ends_with("\n") { "" } else { "\n" };
            writeln!(file, "{}", prefix.to_owned() + &todo).unwrap();
        }
    }
}
