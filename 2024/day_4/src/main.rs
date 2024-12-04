use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    if let Ok(file_iter) = read_lines("input_4.txt") {
        //let total = get_xmas_count(get_input_as_vec(file_iter));
        //println!("Total: {}", total);
        let new_total = get_x_mas_count(get_input_as_vec(file_iter));
        println!("Part 2: {}", new_total);
    }
}

fn get_xmas_count(text_vec: Vec::<String>) -> usize {
    let mut total = 0;
    total += get_horizontal(&text_vec);
    total += get_vertical(&text_vec);
    total += get_diag_left(&text_vec);
    total += get_diag_right(&text_vec);
    return total;
}

fn get_horizontal(text_vec: &Vec::<String>) -> usize {
    let re_forward: Regex = Regex::new(r"XMAS").unwrap();
    let re_reverse: Regex = Regex::new(r"SAMX").unwrap();
    let mut total: usize = 0;
    for line in text_vec {
        total += re_forward.find_iter(line).count();
        total += re_reverse.find_iter(line).count();
    }
    return total;
}

fn get_vertical(text_vec: &Vec::<String>) -> usize {
    let mut vertical_vec: Vec::<String> = Vec::new();
    for j in 0..text_vec[0].len() {
        let mut vert_chars: Vec::<char> = Vec::new();
        for i in 0..text_vec.len() {
            vert_chars.push(text_vec[i].chars().nth(j).unwrap());
        }
        vertical_vec.push(vert_chars.into_iter().collect());
    }
    let re_forward: Regex = Regex::new(r"XMAS").unwrap();
    let re_reverse: Regex = Regex::new(r"SAMX").unwrap();
    let mut total: usize = 0;
    for line in vertical_vec {
        total += re_forward.find_iter(&line).count();
        total += re_reverse.find_iter(&line).count();
    }
    return total;
}

fn get_diag_right(text_vec: &Vec::<String>) -> usize {
    let mut diag_vec: Vec::<String> = Vec::new();
    let max_down = text_vec.len();
    let max_right = text_vec[0].len();
    let mut i = 0;
    let mut j;
    'strbuild: loop {
        let mut diag_chars: Vec::<char> = Vec::new();
        j = 0;
        'charcollect: loop {
            if j >= max_right || i<j {
                break 'charcollect;
            } else if (i-j) >= max_down {
                j += 1;
            } else {
                diag_chars.push(text_vec[i-j].chars().nth(j).unwrap());
                j += 1;
            }
        }
        diag_vec.push(diag_chars.into_iter().collect());
        i += 1;
        if i > max_down+max_right {
            break 'strbuild;
        }
    }
        
    let re_forward: Regex = Regex::new(r"XMAS").unwrap();
    let re_reverse: Regex = Regex::new(r"SAMX").unwrap();
    let mut total: usize = 0;
    for line in diag_vec {
        total += re_forward.find_iter(&line).count();
        total += re_reverse.find_iter(&line).count();
    }
    return total;
}

fn get_diag_left(text_vec: &Vec::<String>) -> usize {
    let mut diag_vec: Vec::<String> = Vec::new();
    let max_down = text_vec.len();
    let max_right = text_vec[0].len();
    let mut i: isize = (max_down - 1).try_into().unwrap();
    let mut j: isize;
    'strbuild: loop {
        let mut diag_chars: Vec::<char> = Vec::new();
        j = 0;
        'charcollect: loop {
            if j >= max_right.try_into().unwrap() || i+j >= max_down.try_into().unwrap() {
                break 'charcollect;
            } else if (i+j) < 0 {
                j += 1;
            } else {
                diag_chars.push(text_vec[TryInto::<usize>::try_into(i+j).unwrap()].chars().nth(j.try_into().unwrap()).unwrap());
                j += 1;
            }
        }
        diag_vec.push(diag_chars.into_iter().collect());
        i -= 1;
        if -1*i > max_right.try_into().unwrap() {
            break 'strbuild;
        }
    }
        
    let re_forward: Regex = Regex::new(r"XMAS").unwrap();
    let re_reverse: Regex = Regex::new(r"SAMX").unwrap();
    let mut total: usize = 0;
    for line in diag_vec {
        total += re_forward.find_iter(&line).count();
        total += re_reverse.find_iter(&line).count();
    }
    return total;
}

fn get_input_as_vec(input_iter: io::Lines<io::BufReader<File>>) -> Vec::<String> {
    let mut output_vec: Vec::<String> = Vec::new();
    for line in input_iter {
        output_vec.push(line.unwrap());
    }
    return output_vec;
}

fn get_x_mas_count(text_vec: Vec::<String>) -> usize {
    let mut total = 0;
    for i in 1..text_vec.len()-1 {
        for j in 1..text_vec[0].len()-1 {
            let curr_line: Vec<char> = text_vec[i].chars().collect();
            let prev_line: Vec<char> = text_vec[i-1].chars().collect();
            let next_line: Vec<char> = text_vec[i+1].chars().collect();
            if curr_line[j] == 'A' {
                let upper_left = prev_line[j-1];
                let upper_right = prev_line[j+1];
                let lower_left = next_line[j-1];
                let lower_right = next_line[j+1];
                match (upper_left, upper_right, lower_left, lower_right) {
                    ('M','M','S','S') | ('M','S','M','S') | ('S','S','M','M') | ('S','M','S','M') => total += 1,
                    _ => continue,
                }
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
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_xmas_count(test_vec), 18);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_x_mas_count(test_vec), 9);
        }
    }

    #[test]
    fn test_horizontal() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_horizontal(&test_vec), 5);
        }
    }

    #[test]
    fn test_vertical() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_vertical(&test_vec), 3);
        }
    }

    #[test]
    fn test_diag_right() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_diag_right(&test_vec), 5);
        }
    }

    #[test]
    fn test_diag_left() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let test_vec = get_input_as_vec(file_iter);
            assert_eq!(get_diag_left(&test_vec), 5);
        }
    }
}