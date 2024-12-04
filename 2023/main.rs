use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total: u64 = 0;
    let re_line_parse: regex::Regex = Regex::new(r"[0-9A-Z]+").unwrap();
    if let Ok(file_iter) = read_lines("input_1.txt") {
        for line in file_iter {
            if let Ok(text) = line {

            }
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