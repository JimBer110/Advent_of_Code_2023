use std::fs;

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;
    for (i, line) in lines.to_owned().enumerate() {
        let mut data: (i8, i8) = (-1, -1);
        for c in line.chars() {
            if c.is_numeric() {
                if data.0 == -1 {
                    data.0 = c.to_digit(10).unwrap() as i8;
                }
                data.1 = c.to_digit(10).unwrap() as i8;
            }
        }
        if data.0 != -1 {
            sum += data.0 as u32 * 10 + data.1 as u32;
        }
    }

    println!("{}", sum);

    sum = 0;
    for (i, line) in lines.enumerate() {
        let mut data: (i8, i8) = (-1, -1);
        for c in line.chars() {
            if c.is_numeric() {
                if data.0 == -1 {
                    data.0 = c.to_digit(10).unwrap() as i8;
                }
                data.1 = c.to_digit(10).unwrap() as i8;
            }
        }
        if data.0 != -1 {
            sum += data.0 as u32 * 10 + data.1 as u32;
        }
    }

    println!("{}", sum);

    std::process::exit(0);
}
