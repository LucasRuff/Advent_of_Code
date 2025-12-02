use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_1.txt") {
        let (stopped_zeroes, total_zeroes) = count_zeroes(file_iter);
        println!("Total zeroes: {}", total_zeroes);
        println!("Stopped zeroes: {}", stopped_zeroes);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn count_zeroes(lines: io::Lines<io::BufReader<File>>) -> (u32, u32) {
    let mut counter = 50;
    let mut stopped_zeroes = 0;
    let mut zeroes = 0;
    let mut new_counter;
       for line in lines {
        if let Ok(text) = line {
            let dir = text.chars().nth(0).unwrap();
            let mut val = text.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
            if dir == 'L' {
                val *= -1;
            }
            while val > 100 {
                zeroes += 1;
                val -= 100;
            }
            while val < -100 {
                zeroes += 1;
                val += 100;
            }

            new_counter = counter + val;
            if new_counter < 0 {
                new_counter += 100;
            }
            new_counter %= 100;
            if new_counter > counter && val < 0 || new_counter < counter && val > 0 {
                zeroes += 1;
            }
            
            counter = new_counter;
            if counter == 0 {
                stopped_zeroes += 1;
            }
            
        }
    }
    (stopped_zeroes, zeroes)
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
            assert_eq!(count_zeroes(file_iter), (3, 6));
        }
    }
}
