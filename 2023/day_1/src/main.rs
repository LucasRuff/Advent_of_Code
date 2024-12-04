use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

const DIGIT_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGIT_NUMS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let mut total: u32 = 0;
    if let Ok(file_iter) = read_lines("input_1.txt") {
        for line in file_iter {
            if let Ok(text) = line {
                let mut calibration: String = String::new();
                let mut first_digit: char = 'a';
                let mut last_digit: char = 'a';
                let mut first_digit_position: usize = usize::MAX;
                let mut last_digit_position: usize = 0;
                // search for words
                for digit_tuple in DIGIT_WORDS.into_iter().zip(DIGIT_NUMS.into_iter()) {
                    match text.find(digit_tuple.0) {
                        Some(pos) => {
                            if pos < first_digit_position {
                                first_digit_position = pos;
                                first_digit = digit_tuple.1;
                            }
                        },
                        None => {},
                    }
                    match text.rfind(digit_tuple.0) {
                        Some(pos) => {
                            if pos > last_digit_position {
                                last_digit_position = pos;
                                last_digit = digit_tuple.1;
                            }
                        },
                        None => {},
                    }
                }
                // search for characters
                for (i, cha) in text.chars().enumerate() {
                    if cha.is_digit(10) {
                        if i < first_digit_position || first_digit == 'a' {
                            first_digit = cha;
                            first_digit_position = i;
                        } 
                        if i > last_digit_position || last_digit == 'a' {
                            last_digit = cha;
                            last_digit_position = i;
                        }
                    }
                }
                calibration.push(first_digit);
                calibration.push(last_digit);
                total += calibration.parse::<u32>().unwrap();
            }
        }
    }
    println!("{}", total);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}