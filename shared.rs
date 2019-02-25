use std::env::{args, var};
use std::io::{stdin, BufRead};

pub fn init_path() -> String {
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
    return path;
}

#[allow(dead_code)]
pub fn get_todos() -> Vec<String> {
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
    return todos;
}

#[allow(dead_code)]
pub fn split_lines(contents: String) -> Vec<String> {
    let lines: Vec<String> = contents
        .trim()
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.to_string())
        .collect();
    return lines;
}
