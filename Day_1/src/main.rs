use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\n");

    // let mut data: Vec<(u16, u16)>;

    for (i, line) in lines.enumerate() {
        for c in line.chars() {
            println!("{}", c);
        }
        // data.push();
        println!("{}: {}", i, line)
    }

    std::process::exit(0);
}
