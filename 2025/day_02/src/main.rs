use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_2.txt") {
        let Ok(file_text) = file_iter.collect::<Result<Vec<String>, _>>() else {
            panic!("Failed to read input");
        };
        let ranges = get_ranges(&file_text[0]);
        let (bad_ids, bad_doubles) = test_ids(&ranges);
        println!("Bad IDs: {}", bad_ids);
        println!("Bad Doubles: {}", bad_doubles);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_ranges(line: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut ranges = Vec::new();
    for cap in re.captures_iter(line) {
        let start = cap[1].parse::<u64>().unwrap();
        let end = cap[2].parse::<u64>().unwrap();
        ranges.push((start, end));
    }
    ranges
}

#[allow(dead_code)]
fn test_ids_part_1(ranges: &Vec<(u64, u64)>) -> u64 {

    let mut bad_ids = 0;
    for range in ranges {
        let mut current_id = range.0;
        while current_id <= range.1 {
            let digits = get_num_digits(current_id);
            if digits % 2 != 0 {
                current_id += 1;
                continue;
            }
            let mut radix = "1".to_string();
            for _ in 0..digits/2 {
                radix.push('0');
            }
            //radix.push_str(radix.clone().as_str());
            let radix_num = radix.parse::<u64>().unwrap();
            //println!("Current ID: {}, Radix: {}", current_id, radix_num);
            let top_half = current_id / radix_num;
            let bottom_half = current_id - top_half * radix_num;
            //println!("Top half: {}, Bottom half: {}", top_half, bottom_half);
            if top_half == bottom_half {
                bad_ids += current_id;
                //println!("Bad ID found: {}", current_id);
            }
            current_id += 1;
        }
    }
    bad_ids
}

fn test_ids(ranges: &Vec<(u64, u64)>) -> (u64, u64) {

    let mut bad_ids = 0;
    let mut bad_doubles = 0;
    for range in ranges {
        let mut current_id = range.0;
        
        'id_list: while current_id <= range.1 {
            
            let digits = get_num_digits(current_id);
            let current_id_str = current_id.to_string();
            let divisors = get_all_divisors(digits);
            'divisor_list: for divisor in divisors.iter().rev() {
                
                for i in 0..*divisor {
                    let first_char = current_id_str.chars().nth(i as usize).unwrap();
                    let mut j = i + *divisor;
                    while j < digits {
                        let compare_char = current_id_str.chars().nth(j as usize).unwrap();
                        if compare_char != first_char {
                            continue 'divisor_list;
                        }
                        j += *divisor;
                    }
                }
                
                bad_ids += current_id;
                if *divisor == digits / 2 && digits % 2 == 0 {
                    bad_doubles += current_id;
                    
                }
                current_id += 1;
                continue 'id_list;
                    
            }
            current_id += 1;
        }
    }
    (bad_ids, bad_doubles)
}

fn get_all_divisors(id: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in 1..=(id/2) {
        if id % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn get_num_digits(mut id: u64) -> u64 {
    let mut digits = 0;
    while id > 0 {
        id /= 10;
        digits += 1;
    }
    digits
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
            let Ok(input_text) = file_iter.collect::<Result<Vec<String>, _>>() else {
                panic!("Failed to read test input");
            };
            let ranges = get_ranges(&input_text[0]);
            assert_eq!(test_ids(&ranges), (4174379265, 1227775554));
        }
    }
    #[test]
    fn divisor_test() {
        let divisors = get_all_divisors(28);
        assert_eq!(divisors, vec![1, 2, 4, 7, 14]);
    }
}
