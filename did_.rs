use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let db_path = env::var("HOME").unwrap() + "/.todo_db";
    let db_exists = fs::metadata(&db_path).is_ok();
    if args.len() > 1 {
        args.remove(0);
        if db_exists {
            let todo = args.join(" ");
            let db = fs::read_to_string(&db_path).unwrap();
            let old_todos: Vec<&str> = db.split("\n").collect();
            let old_len = old_todos.len();
            let new_todos: Vec<&str> = old_todos.into_iter().filter(|x| x != &todo).collect();
            fs::write(&db_path, new_todos.join("\n")).unwrap();
            if new_todos.len() < old_len {
                println!("\x1B[31m- {}\x1B[0m", todo);
            }
        }
    }
}
