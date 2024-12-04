use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    
    if let Ok(file_iter) = read_lines("input_3.txt") {
        let total_1 = get_uncorrupted(file_iter);
        println!("{}", total_1);
    }
    if let Ok(file_iter) = read_lines("input_3.txt") {
        let total_2 = get_part_2(file_iter);
        println!("{}", total_2);
    }
}

fn get_uncorrupted(file: io::Lines<io::BufReader<File>>) -> u32 {
    let re_mul: Regex = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
    
    let mut total = 0;
    for line in file {
        let text = line.unwrap();
        let muls: Vec::<u32> = re_mul.captures_iter(&text).map(|cap| {
            let (_, [a, b]) = cap.extract();
            let a_int = a.parse::<u32>().unwrap();
            let b_int = b.parse::<u32>().unwrap();
            a_int * b_int
        }).collect();
        for mul in muls {
            total += mul;
        }
    }
    return total;
}

fn get_part_2(file: io::Lines<io::BufReader<File>>) -> u32 {
    let patterns: Regex = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();
    let mut total = 0;
    let mut is_doing = true;
    for line in file {
        let text = line.unwrap();
        let matches = patterns.captures_iter(&text);
        for regex_match in matches {
                       
            if regex_match.name("do").is_some() {
                is_doing = true;
                continue;
            } else if  regex_match.name("dont").is_some() {
                    is_doing = false;
                    continue;
            } else if is_doing {
                let a = regex_match.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
                let b = regex_match.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
                total += a * b;
            }
        }
    }
    return total;
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
            assert_eq!(get_uncorrupted(file_iter),161);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let res = get_part_2(file_iter);
            assert_eq!(res, 48);
        }
    }

    #[test]
    fn test_regex() {
        let hay = "asdfmul(3,2)";
        let re: Regex = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap(); 
        let caps: Vec::<(u32,u32)> = re.captures_iter(hay).map(|cap| {
            let (_, [a, b]) = cap.extract();
            let new_a = a.parse::<u32>().unwrap();
            let new_b = b.parse::<u32>().unwrap();
            println!("A and B: {} {}", new_a, new_b);
            (new_a,new_b)
        }).collect();
        assert_eq!(vec![(3,2)], caps);
    }

    #[test]
    fn test_do_regex() {
        let hay = "asdfdo()d";
        let re: Regex = Regex::new(r"do\(\)").unwrap();
        
        assert!(re.is_match(hay));
    }

    #[test]
    fn test_dont_regex() {
        let hay = "asdfdo()dont()don't()";
        let re: Regex = Regex::new(r"don\'t\(\)").unwrap();
        
        assert!(re.is_match(hay));
    }

    #[test]
    fn test_regex_set() {
        let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let patterns: Regex = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();
        let matches: Vec::<_> = patterns.captures_iter(hay).map(|cap| {
            let mut cap_type = 0;
            let my_cap = cap.name("do");
            if my_cap.is_some() {
                cap_type = 1;
            }
            let my_cap = cap.name("dont");
            if my_cap.is_some() {
                cap_type = 2;
            }
            cap_type
        }).collect();
        assert_eq!(vec![0,2,0,0,1,0], matches);
    }
}