use std::env;
use std::fs;

fn is_number(string: String, numbers: &[&str]) -> i8 {
    for (i, x) in numbers.into_iter().enumerate() {
        if string.contains(x) {
            return i as i8 + 1;
        }
    }
    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;
    for (_i, line) in lines.to_owned().enumerate() {
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
    for (_i, line) in lines.enumerate() {
        let mut tmp: String = String::new();
        let mut data: (i8, i8) = (-1, -1);
        for c in line.chars() {
            tmp.push(c);
            let res_of_contains = is_number(tmp.to_owned(), &numbers);
            if res_of_contains != -1 {
                if data.0 == -1 {
                    data.0 = res_of_contains;
                }
                data.1 = res_of_contains;
                tmp = String::from(
                    &numbers[res_of_contains as usize - 1][numbers[res_of_contains as usize - 1]
                        .char_indices()
                        .nth_back(1)
                        .unwrap()
                        .0..],
                );
            } else if c.is_numeric() {
                if data.0 == -1 {
                    data.0 = c.to_digit(10).unwrap() as i8;
                }
                data.1 = c.to_digit(10).unwrap() as i8;
                tmp = String::from("");
            }
        }
        if data.0 != -1 {
            sum += data.0 as u32 * 10 + data.1 as u32;
        }
    }

    println!("{}", sum);

    std::process::exit(0);
}
