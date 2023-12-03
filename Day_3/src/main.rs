use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        data.push(line.chars().collect());
    }

    for i in 0..data.len() {
        for j in 0..data[i].len() {}
    }
}
