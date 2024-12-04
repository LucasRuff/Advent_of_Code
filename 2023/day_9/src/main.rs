use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let re_nums: regex::Regex = Regex::new(r"[-]*[0-9]+").unwrap();
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_9.txt") {
        let mut ends_total: i64 = 0;
        let mut starts_total: i64 = 0;
        for line in file_iter {
            if let Ok(text) = line {
                let mut parsed_text: Vec<i64> = re_nums.find_iter(&text).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
                let mut rows: Vec<Vec<i64>> = vec![parsed_text.clone()];
                let mut diffs: Vec<i64> = get_diffs(&parsed_text);
                let mut line_ends: Vec<i64> = vec![0];
                let mut line_starts: Vec<i64> = vec![0];
                
                while !all_zero(&diffs) {
                    rows.push(diffs.clone());
                    diffs = get_diffs(&diffs);
                }
                rows.reverse();
                for (i, row) in rows.clone().iter().enumerate() {
                    //println!("Evaluating row {:?}", row);
                    let old_end = row[row.len()-1];
                    let old_start = row[0];
                    let prev_end = line_ends[i];
                    let prev_start = line_starts[i];
                    //println!("Adding {}+{} to line", old_end, prev_end);
                    line_ends.push(row[row.len()-1] + line_ends[i]);
                    line_starts.push(row[0] - line_starts[i]);
                }
                //println!("next num: {}", line_ends[line_ends.len()-1]);
                ends_total += line_ends[line_ends.len()-1];
                starts_total += line_starts[line_starts.len()-1];
            }
        }
        println!("ends total: {}", ends_total);
        println!("starts total: {}", starts_total);
        println!("time: {:?}", now.elapsed());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_diffs(input_vector: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    let mut input_nums_iter = input_vector.clone().into_iter();
    let mut first_num = input_nums_iter.next().unwrap();
    for second_num in input_nums_iter {
        result.push(second_num - first_num);
        first_num = second_num;
    }
    return result;
}

fn all_zero(input_vector: &Vec<i64>) -> bool {
    for input_num in input_vector.clone() {
        if input_num != 0 { return false; }
    }
    true
}