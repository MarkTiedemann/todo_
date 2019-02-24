use std::env::{args, var};
use std::fs::{metadata, read_to_string, write};

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
        let mut args: Vec<String> = args().collect();
        let remove = args.len() > 1;
        if remove {
            args.remove(0);
        }
        let joined_args = args.join(" ");
        let todo = joined_args.trim();

        let contents = read_to_string(&path).unwrap();

        let lines: Vec<&str> = contents
            .trim()
            .split("\n")
            .filter(|line| {
                if line.trim().len() == 0 {
                    false
                } else if remove && line == &todo {
                    println!("\x1B[31m- {}\x1B[0m", todo);
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
