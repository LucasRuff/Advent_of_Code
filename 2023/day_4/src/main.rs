use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let mut total: u32 = 0;
    let mut card_copies: [u32; 209] = [1; 209];
    let re_number: regex::Regex = Regex::new(r"[0-9]+").unwrap();
    if let Ok(file_iter) = read_lines("input_4.txt") {
        for line in file_iter {
            if let Ok(text) = line {
                let part_1 = false;
                let card_id = re_number.find(&text[..9]).unwrap().as_str().parse::<u32>().unwrap();
                let mut card_parts = text.split("|");
                let my_nums_text = &card_parts.next().unwrap()[9..];
                let card_nums_text = &card_parts.next().unwrap();
                let my_nums: Vec<u32> = re_number.find_iter(my_nums_text).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
                let card_nums: Vec<u32> = re_number.find_iter(card_nums_text).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
                let mut matches: u32 = 0;
                for my_num in my_nums.clone().into_iter() {
                    for card_num in card_nums.clone().into_iter() {
                        if my_num == card_num {
                            matches += 1;
                        }
                    }
                }
                if part_1 {
                    if matches > 0 {
                        total += 2_u32.pow(matches - 1);
                    }
                } else {
                    if matches > 0 {
                        let indices: Vec<u32> = (0..matches).collect();
                        for index in indices {
                            card_copies[(card_id + index) as usize] += card_copies[(card_id-1) as usize];
                        }
                    }
                }
            }
        }
    }
    total = card_copies.iter().sum();
    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}