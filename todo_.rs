mod shared;

use shared::{init_path, split_lines};
use std::fs::{metadata, read_to_string};

fn main() {
    let path = init_path();
    if metadata(&path).is_ok() {
        let contents = read_to_string(&path).unwrap();
        for line in split_lines(contents) {
            println!("• {}", line);
        }
    }
}
