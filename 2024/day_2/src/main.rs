use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    if let Ok(file_iter) = read_lines("input_2.txt") {
        let total_safe = count_safe_reports(file_iter, true);
        println!("Total: {}", total_safe);
    }
}

fn count_safe_reports(reports: io::Lines<io::BufReader<File>>, dampener_enabled: bool) -> u32 {
    let mut safe_count = 0;
    for report in reports {
        if let Ok(report_text) = report {
            let levels: Vec::<&str> = report_text.split(" ").collect();
            if dampener_enabled {
                if is_report_safe(levels, false) {
                    safe_count += 1;
                }
            } else {
                if is_report_safe(levels, true) {
                    safe_count += 1;
                }
            }
        }
    }
    return safe_count;
}

fn is_report_safe(levels: Vec::<&str>, dampener_used: bool) -> bool {
    let num_levels = levels.len();
    let mut level_iter = levels.iter();
    let mut first_level;
    'get_start: loop{
        let starter = level_iter.next().unwrap().parse::<i32>();
        match starter {
            Ok(k) => {
                first_level = k;
                break 'get_start;
            },
            Err(_) => continue 'get_start,
        }
    }
    let t = level_iter.next().unwrap();
    let mut second_level = t.parse::<i32>().unwrap();
    let ascending: bool = first_level > second_level;
    loop {
        let step_size = (first_level - second_level).abs();
        if step_size == 0 || step_size > 3 || (first_level > second_level) != ascending {
            if dampener_used {
                return false;
            } else {
                for i in 0..num_levels {
                    let mut second_half: Vec::<&str> = levels[i+1..].to_vec();
                    let mut test_levels: Vec::<&str> = levels.clone();
                    test_levels.truncate(i);
                    
                    test_levels.append(&mut second_half);
                    println!("Testing new levels {:?}", test_levels);
                    if is_report_safe(test_levels, true) {
                        return true;
                    }
                }
                return false;
            }
        }

        first_level = second_level;
        let temp = level_iter.next();
        match temp {
            Some(t) => second_level = t.parse::<i32>().unwrap(),
            None => return true,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(count_safe_reports(file_iter, false), 2);
        }
    }
    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(count_safe_reports(file_iter, true), 4);
        }
    }
}