use std::env::{args, var};
use std::fs::{metadata, read_to_string, write};
use std::io::{stdin, BufRead};

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

    if metadata(&path).is_ok() {
        let todos: Vec<String>;
        let args = args();
        if args.len() > 1 {
            let mut words: Vec<String> = args.collect();
            words.remove(0);
            let joined = words.join(" ");
            let todo = joined.trim();
            todos = vec![todo.to_string()];
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
                .collect();
        }

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
