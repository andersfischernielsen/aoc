mod day_1;
mod day_2;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = read_file(path);
    let day_2 = day_2::calculate(&contents);
    println!("{}", day_2);
}

fn read_file(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}
