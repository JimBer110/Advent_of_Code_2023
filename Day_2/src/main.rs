use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum: u16 = 0;
    let mut sum_2: u32 = 0;
    for (i, line) in contents.lines().enumerate() {
        let mut part_2 = [0; 3];
        let mut games: Vec<[u16; 3]> = Vec::new();

        let text = line.split(": ").nth(1).unwrap();

        let parts = text.split("; ");

        for part in parts {
            let mut data = [0; 3];
            for item in part.split(", ") {
                let itemized = item.split(" ").collect::<Vec<&str>>();
                let size = itemized[0].parse::<u16>().unwrap();
                if itemized[1] == "red" {
                    data[0] = size;
                } else if itemized[1] == "green" {
                    data[1] = size;
                } else {
                    data[2] = size;
                }
            }
            games.push(data);
        }

        let mut possible = true;
        for game in games {
            if game[0] > 12 || game[1] > 13 || game[2] > 14 {
                possible = false;
            }
            if game[0] > part_2[0] {
                part_2[0] = game[0];
            }
            if game[1] > part_2[1] {
                part_2[1] = game[1];
            }
            if game[2] > part_2[2] {
                part_2[2] = game[2];
            }
        }

        // println!(" {}", text);

        if possible {
            sum += i as u16 + 1;
        }
        let mut tmp = 1;
        for n in 0..3 {
            tmp *= part_2[n];
        }
        sum_2 += tmp as u32;
    }
    println!("{}", sum);
    println!("{}", sum_2);
}
