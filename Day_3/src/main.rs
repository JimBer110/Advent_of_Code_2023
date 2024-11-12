use std::env;
use std::fs;

#[derive(Debug)]
struct engine_part {
    data: String,
    value: u32,
    is_part: bool,
    location: (u32, u32),
    part: char,
}

impl engine_part {
    fn new() -> Self {
        return Self {
            data: String::new(),
            value: 0,
            is_part: false,
            location: (0, 0),
            part: '.',
        };
    }

    fn check_around(&mut self, data: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let directions = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        for (di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && ni < data.len() as i32 && nj >= 0 && nj < data[0].len() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if !data[ni][nj].is_ascii_digit() && data[ni][nj] != '.' {
                    self.part = data[ni][nj];
                    self.location = (ni as u32, nj as u32);
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        data.push(line.chars().collect());
    }

    let mut engine_parts: Vec<engine_part> = Vec::new();

    for i in 0..data.len() {
        let mut part: engine_part = engine_part::new();
        for j in 0..data[i].len() {
            if data[i][j].is_ascii_digit() {
                part.data.push(data[i][j]);
                if part.check_around(&data, i, j) {
                    part.is_part = true;
                }
                if j != data[i].len() - 1 {
                    continue;
                }
            }
            if !part.data.is_empty() {
                part.value = part.data.parse::<u32>().unwrap();
                engine_parts.push(part);
                part = engine_part::new();
            }
        }
    }

    let mut sum = 0;
    let mut sum2 = 0;

    for part in engine_parts.iter() {
        if part.is_part {
            sum += part.value;
            if part.part == '*' {
                let tmp_parts: Vec<&engine_part> = engine_parts
                    .iter()
                    .filter(|x| x.location.0 == part.location.0 && x.location.1 == part.location.1)
                    .collect();

                if tmp_parts.len() == 2 {
                    sum2 += tmp_parts[0].value * tmp_parts[1].value;
                }
            }
        }
    }
    sum2 /= 2;

    println!("{}", sum);
    println!("{}", sum2)
}
