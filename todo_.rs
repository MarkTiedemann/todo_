use std::env::var;
use std::fs::{metadata, read_to_string};

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
