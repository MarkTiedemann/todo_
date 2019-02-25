mod shared;

use shared::{get_todos, init_path, split_lines};
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let path = init_path();
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = split_lines(contents.clone());
    for line in &lines {
        println!("• {}", line);
    }
    let todos = get_todos();
    if todos.len() > 0 {
        for todo in &todos {
            println!("\x1B[32m+ {}\x1B[0m", todo);
        }
        let prefix = if contents.ends_with("\n") { "" } else { "\n" };
        let new_contents = prefix.to_owned() + &todos.join("\n") + "\n";
        writeln!(file, "{}", new_contents).unwrap();
    }
}
