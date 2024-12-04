use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total: isize = 0;
    if let Ok(file_iter) = read_lines("input_18_test.txt") {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut current_location = (0,0);
        let mut visited_locations: Vec<(isize, isize)> = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                let split_text: Vec<&str> = text.split(' ').collect();
                let direction = match split_text[0] {
                    "U" => Direction::North,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    "R" => Direction::East,
                    _ => panic!("Err: Could not read direction"),
                };
                let distance = split_text[1].parse::<isize>().unwrap();
                let color = Color{ r: usize::from_str_radix(&split_text[2][2..4], 16).unwrap(), g: usize::from_str_radix(&split_text[2][4..6], 16).unwrap(), b: usize::from_str_radix(&split_text[2][6..8], 16).unwrap() };
                instructions.push(Instruction{ direction, distance, color });
            }
        }
        for instr in instructions.iter() {
            match instr.direction {
                Direction::East => {
                    current_location = (current_location.0 + instr.distance, current_location.1);
                },
                Direction::North => {
                    current_location = (current_location.0, current_location.1 + instr.distance);
                },
                Direction::South => {
                    current_location = (current_location.0, current_location.1 - instr.distance);
                },
                Direction::West => {
                    current_location = (current_location.0 - instr.distance, current_location.1);
                },
            }
            visited_locations.push(current_location);
        }
        let mut min_y = 0;
        let mut min_x = 0;
        for locale in visited_locations.iter() {
            if locale.0 < min_x { min_x = locale.0 };
            if locale.1 < min_y { min_y = locale.1 };
        }
        println!("Min x: {}, min y: {}", min_x, min_y);
        for i in 0..visited_locations.len() {
            visited_locations[i].0 -= min_x;
            visited_locations[i].1 -= min_y;
            println!("New location: ({}, {})", visited_locations[i].0, visited_locations[i].1);
        }
        let mut prev_locale = visited_locations[visited_locations.len()-1];
        for (i, instr) in instructions.iter().enumerate() {
            let current_locale = visited_locations[i];
            match instr.direction {
                Direction::East => {
                    total += (prev_locale.1 + 1) * (current_locale.0 - prev_locale.0 + 1);
                },
                Direction::North => {},
                Direction::South => {
                    total += (hy)
                },
                Direction::West => {
                    total -= (prev_locale.1 + 1) * (prev_locale.0 - current_locale.0 + 1);
                },
            }
            prev_locale = current_locale;
        }
    }

    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Instruction {
    direction: Direction,
    distance: isize,
    color: Color,
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Color {
    r: usize,
    g: usize,
    b: usize,
}