use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_6.txt") {
        let (numbers, operations) = process_all_lines(file_iter);
        let result = perform_operations_on_numbers(numbers, operations);
        println!("The final result is: {}", result);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(line)
        .filter_map(|digits| digits.as_str().parse::<u64>().ok())
        .collect()
}

fn get_icons_from_line(line: &str) -> Vec<char> {
    line.chars().filter_map(|c| if c == '+' || c == '*' { Some(c) } else { None }).collect()
}

fn process_all_lines(lines: io::Lines<io::BufReader<File>>) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let mut nums_in_line = get_numbers_from_line(&ip);
            if nums_in_line.is_empty() {
                operations.extend(get_icons_from_line(&ip));
                continue;
            }
            numbers.push(nums_in_line);
        }
    }
    (numbers, operations)
}

fn perform_operations_on_numbers(number_lists: Vec<Vec<u64>>, operation_list: Vec<char>) -> u64 {
    let mut result = 0;
    let mut operations_pointer = 0;
    for col_number in 0..number_lists[0].len() {
        
        match operation_list[operations_pointer] {
            '+' => {
                let mut intermediate_result = 0;
                for row_number in 0..number_lists.len() {
                    intermediate_result += number_lists[row_number][col_number];
                }
                //println!("Intermediate sum: {}", intermediate_result);
                result += intermediate_result;
            }
            '*' => {
                let mut intermediate_result = 1;
                for row_number in 0..number_lists.len() {
                    intermediate_result *= number_lists[row_number][col_number];
                }
                //println!("Intermediate product: {}", intermediate_result);
                result += intermediate_result;
            }
            _ => {}
        }
        operations_pointer += 1;
    }
    result
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
            let (numbers, operations) = process_all_lines(file_iter);
            let result = perform_operations_on_numbers(numbers, operations);
            assert_eq!(result, 4277556);
        }

    }
}
