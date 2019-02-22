use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let db_path = env::var("HOME").unwrap() + "/.todo_db";
    let db_exists = fs::metadata(&db_path).is_ok();
    if db_exists {
        let db = fs::OpenOptions::new().read(true).open(&db_path).unwrap();
        for todo in io::BufReader::new(db).lines() {
            println!("* {}", todo.unwrap());
        }
    }
}
