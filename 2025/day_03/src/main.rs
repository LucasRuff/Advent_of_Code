use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_3.txt") {
        let total_joltage = get_total_joltage(file_iter);
        println!("Total joltage: {}", total_joltage);
        let file_iter = read_lines("input_3.txt").unwrap();
        let overload_joltage = get_overload_joltage(file_iter);
        println!("Overload joltage: {}", overload_joltage);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_total_joltage(lines: io::Lines<io::BufReader<File>>) -> usize {
    let mut total = 0;
    for line in lines {
        if let Ok(text) = line {
            total += get_line_joltage(&text);
        }
    }
    total
}

fn get_line_joltage(line: &str) -> usize {
    let line_chars = line.chars().collect::<Vec<char>>();
    let mut first_digit = line_chars[0].to_digit(10).unwrap() as usize;
    let mut second_digit = line_chars[1].to_digit(10).unwrap() as usize;
    let mut high_number = first_digit * 10 + second_digit;
    for char in 2..line_chars.len()-1 {
        let digit = line_chars[char].to_digit(10).unwrap() as usize;
        if digit > first_digit {
            first_digit = digit;
            second_digit = 0;
            high_number = first_digit * 10 + second_digit;
        } else if first_digit * 10 + digit > high_number {
            second_digit = digit;
            high_number = first_digit * 10 + digit;
        }
        
    }
    let last_digit = line_chars[line_chars.len()-1].to_digit(10).unwrap() as usize;
    if last_digit > second_digit {
        second_digit = last_digit;
    }
    first_digit * 10 + second_digit
}

fn get_overload_joltage(lines: io::Lines<io::BufReader<File>>) -> usize {
    let mut total = 0;
    for line in lines {
        if let Ok(text) = line {
            total += get_line_overload_joltage(&text);
        }
    }
    total
}

fn get_line_overload_joltage(line: &str) -> usize {
    let line_chars = line.chars().collect::<Vec<char>>();
    let mut digits = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    for char in 0..line_chars.len() {
        let digit = line_chars[char].to_digit(10).unwrap() as usize;
        for d in 0..digits.len() {
            if digit > digits[d] && line_chars.len() - char > (11 - d) {
                digits[d] = digit;
                for u in (d+1)..digits.len() {
                    digits[u] = 0;
                }
                break;
            }
        }
    }
    let mut joltage = 0;
    for digit in digits.iter() {
        joltage *= 10;
        joltage += digit;
    }
    joltage
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(get_total_joltage(file_iter), 357);
        }
    }
    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(get_overload_joltage(file_iter), 3121910778619);
        }
    }
}
