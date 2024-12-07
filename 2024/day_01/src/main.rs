use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(file_iter) = read_lines("input_1.txt") {
        //let total = calculate_total(file_iter);
        //println!("Total: {}", total);
        let similarity = get_similarity(file_iter);
        println!("Similarity: {}", similarity);
    }
}

#[allow(dead_code)]
fn calculate_total(file_iterator: io::Lines<io::BufReader<File>>) -> i32 {
    let mut total_inter: i32 = 0;
    let mut left_list: Vec<i32>;
    let mut right_list: Vec<i32>;
    (left_list, right_list) = get_lists(file_iterator);
    left_list.sort();
    right_list.sort();
    for i in 0..left_list.len() {
        total_inter += (left_list[i] - right_list[i]).abs();
    }
    return total_inter;
}

fn get_similarity(file_iterator: io::Lines<io::BufReader<File>>) -> i32 {
    let mut similarity_inter: i32 = 0;
    let mut left_list: Vec<i32>;
    let mut right_list: Vec<i32>;
    (left_list, right_list) = get_lists(file_iterator);
    left_list.sort();
    right_list.sort();

    for num in left_list.iter() {
        let mut instance_counter = 0;
        for matcher in right_list.iter() {
            if num == matcher {
                instance_counter += 1;
            }
        }
        similarity_inter += instance_counter * num;
    }
    return similarity_inter;
}

fn get_lists(file_iterator: io::Lines<io::BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list_builder: Vec<i32> = Vec::new();
    let mut right_list_builder: Vec<i32> = Vec::new();
    for line in file_iterator {
        if let Ok(text) = line {
            let mut line_parts = text.split(" ");
            left_list_builder.push(line_parts.next().unwrap().parse::<i32>().unwrap());
            right_list_builder.push(line_parts.last().unwrap().parse::<i32>().unwrap());
        }
    }
    return (left_list_builder, right_list_builder);
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
            assert_eq!(calculate_total(file_iter), 11);
        }
    }
    #[test]
    fn get_sides_test() {
        let target_left = Vec::<i32>::from([3, 4, 2, 1, 3, 3]);
        let target_right = Vec::from([4, 3, 5, 3, 9, 3]);
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(get_lists(file_iter), (target_left, target_right));
        }
    }
    #[test]
    fn get_similarity_test() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(get_similarity(file_iter), 31);
        }
    }
}
