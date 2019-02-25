use std::env::{args, var};
use std::fs::OpenOptions;
use std::io::{stdin, BufRead};
use std::io::{Read, Write};

fn main() {
    let home = if cfg!(windows) {
        var("HOMEDRIVE").unwrap() + &var("HOMEPATH").unwrap()
    } else {
        var("HOME").unwrap()
    };
    let default_path = if cfg!(windows) {
        home.clone() + "\\.todo_list"
    } else {
        home.clone() + "/.todo_list"
    };
    let path = var("TODO_LIST").unwrap_or(default_path);
    if var("TODO_PRINT_PATH").is_ok() {
        println!("\x1B[35m{}\x1B[0m", path.replace(&home, "~"));
    }

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

    let todos: Vec<String>;
    let args = args();
    if args.len() > 1 {
        let mut words: Vec<String> = args.collect();
        words.remove(0);
        let joined = words.join(" ");
        let todo = joined.trim();
        if lines.contains(&todo) {
            todos = vec![];
        } else {
            todos = vec![todo.to_string()];
        }
    } else {
        let stdin = stdin();
        todos = stdin
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.trim().to_string())
            .map(|todo| {
                if todo.starts_with("• ") {
                    // • is 3 bytes long
                    todo.get(4..).unwrap().to_string()
                } else {
                    todo
                }
            })
            .filter(|todo| !lines.contains(&&todo[..]))
            .collect();
    }

    if todos.len() > 0 {
        for todo in &todos {
            println!("\x1B[32m+ {}\x1B[0m", todo);
        }
        let prefix = if contents.ends_with("\n") { "" } else { "\n" };
        let new_contents = prefix.to_owned() + &todos.join("\n") + "\n";
        writeln!(file, "{}", new_contents).unwrap();
    }
}
