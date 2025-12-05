use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_4.txt") {
        let lines = get_lines_as_vec(file_iter);
        let (ranges, range_count) = get_ranges(&lines);
        let squashed_ranges = squash_ranges(&ranges);
        let ids = get_ids(&lines, range_count);
        let id_counts = count_ids_in_ranges(&ids, &squashed_ranges);
        println!("Number of IDs in ranges: {}", id_counts);
        let total_ranges = total_good_ranges(&squashed_ranges);
        println!("Total good ranges: {}", total_ranges);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_lines_as_vec(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            vec.push(ip);
        }
    }
    vec
}

fn get_ranges(lines: &Vec<String>) -> (Vec<Range>, usize) {
    let mut ranges: Vec<Range> = Vec::new();
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut count = 0;
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        for cap in re.captures_iter(&line) {
            let low: u64 = cap[1].parse().unwrap();
            let high: u64 = cap[2].parse().unwrap();
            ranges.push(Range { low, high });
        }
        count += 1;
    }
    (ranges, count)
}

fn squash_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.low.cmp(&b.low));
    let mut squashed_ranges: Vec<Range> = Vec::new();
    let mut current_range = sorted_ranges[0].clone();
    for range in sorted_ranges.iter().skip(1) {
        if range.low <= current_range.high {
            if range.high > current_range.high {
                current_range.high = range.high;
            }
        } else {
            squashed_ranges.push(current_range);
            current_range = range.clone();
        }
    }
    squashed_ranges.push(current_range);
    squashed_ranges
}

fn get_ids(lines: &Vec<String>, range_count: usize) -> Vec<u64> {
    let mut ids: Vec<u64> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    let line_iter = lines.iter();
    for line in line_iter.skip(range_count) {
        for cap in re.captures_iter(&line) {
            let id: u64 = cap[1].parse().unwrap();
            ids.push(id);
        }
    }
    ids
}

fn count_ids_in_ranges(ids: &Vec<u64>, ranges: &Vec<Range>) -> u64 {
    let mut id_counts = 0;
    for id in ids.iter() {
        for range in ranges.iter() {
            if *id >= range.low && *id <= range.high {
                id_counts += 1;
                break;
            }
        }
    }
    id_counts
}

fn total_good_ranges(ranges: &Vec<Range>) -> u64 {
    let mut total = 0;
    for range in ranges.iter() {
        total += range.high - range.low + 1;
    }
    total
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct Range {
    low: u64,
    high: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let lines = get_lines_as_vec(file_iter);
            let (ranges, range_count) = get_ranges(&lines);
            let squashed_ranges = squash_ranges(&ranges);
            assert_eq!(squashed_ranges.len(), 2);
            let ids = get_ids(&lines, range_count);
            assert_eq!(ranges.len(), 4);
            assert_eq!(ids.len(), 6);
            let id_counts = count_ids_in_ranges(&ids, &squashed_ranges);
            assert_eq!(id_counts, 3);
        }
    }
    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input_2.txt") {
            let lines = get_lines_as_vec(file_iter);
            let (ranges, _) = get_ranges(&lines);
            let squashed_ranges = squash_ranges(&ranges);
            let total_ranges = total_good_ranges(&squashed_ranges);
            assert_eq!(total_ranges, 14);
        }
    }
}
