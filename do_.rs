use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let db_path = env::var("HOME").unwrap() + "/.todo_db";
    let db_exists = fs::metadata(&db_path).is_ok();
    if args.len() > 1 {
        args.remove(0);
        let mut db = fs::OpenOptions::new()
            .create_new(!db_exists)
            .append(true)
            .open(&db_path)
            .unwrap();
        let todo = args.join(" ");
        writeln!(db, "{}", todo).unwrap();
        println!("\x1B[32m+ {}\x1B[0m", todo);
    }
}
