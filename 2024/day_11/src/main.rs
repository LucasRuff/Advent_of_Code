use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(mut file_iter) = read_lines("input_11.txt") {
        let txt = file_iter.next().unwrap().unwrap();
        let mut total1 = 0;
        let mut total2 = 0;
        let mut blink_memo = HashMap::new();
        for stone_txt in txt.clone().split(" ") {
            let stone = stone_txt.parse::<u128>().unwrap();
            total1 += blink_stone_short(stone, 0);
        }
        println!("total 1: {}", total1);
        println!("Finished in {:?}", now.elapsed());
        for stone_txt in txt.clone().split(" ") {
            let stone = stone_txt.parse::<u128>().unwrap();
            let res_total = blink_stone_recurse(stone, 0, 75, &mut blink_memo, 0);
            total2 += res_total;
        }
        println!("total 2: {}", total2);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_split(parent_stone: u128, max_depth: usize) -> (usize, Option<Vec<u128>>) {
    let mut steps_taken = 0;
    let mut moving_val = parent_stone;
    let mut num_digits = if moving_val != 0 {(moving_val.ilog10() + 1).try_into().unwrap()} else {1};
    while steps_taken <= max_depth {
        steps_taken += 1;
        match (num_digits % 2 == 0, moving_val == 0) {
            (false,true) => {
                moving_val = 1;
            },
            (true,_) => {
                let (stone_1, stone_2) = (moving_val / 10_u128.pow(num_digits / 2), moving_val % (10_u128.pow(num_digits / 2)));
                return (steps_taken, Some(vec![stone_1, stone_2]));

            },
            (false,false) => {
                moving_val = moving_val * 2024;
            },
        }
        num_digits = if moving_val != 0 {(moving_val.ilog10() + 1).try_into().unwrap()} else {1};
    }
    return (max_depth, None);
}

fn blink_stone_recurse(parent_stone: u128, depth: usize, max_depth: usize, start_memo: &mut HashMap<(u128, usize), usize>, iter_depth: usize) -> usize {
    if depth == max_depth {
        return 1;
    }
    match start_memo.get(&(parent_stone, depth)) {
        Some(score) => {
            return *score;
        },
        None => {
            let (split_depth, children) = find_first_split(parent_stone, max_depth);
            if split_depth + depth > max_depth {
                start_memo.insert((parent_stone, depth),1);
                return 1;
            }
            let mut total = 0;
            if children.is_some() {
                for child in children.unwrap().iter() {
                    let child_score = blink_stone_recurse(*child, split_depth + depth, max_depth, start_memo, iter_depth + 1);
                    total += child_score;
                }

                start_memo.insert((parent_stone, depth), total);
                return total;
            }
            start_memo.insert((parent_stone, depth), 1);
            return 1;
        },
    }
}

fn blink_stone_short(stone: u128, depth: usize) -> usize {
    if depth == 25 {
        return 1;
    } else {
        if stone == 0 {
            return blink_stone_short(1, depth + 1);
        }
        let num_digits: u32 = (stone.ilog10() + 1).try_into().unwrap();
        if num_digits % 2 == 0 {
            return blink_stone_short(stone / (10_u128.pow(num_digits / 2)), depth + 1) + blink_stone_short(stone % (10_u128.pow(num_digits / 2)), depth + 1);
        }
        return blink_stone_short(stone * 2024, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let txt = file_iter.next().unwrap().unwrap();
            let mut total = 0;
            for stone_txt in txt.split(" ") {
                let stone = stone_txt.parse::<u128>().unwrap();
                total += blink_stone_short(stone, 0);
            }
            assert_eq!(total, 55312);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let txt = file_iter.next().unwrap().unwrap();
            let mut total = 0;
            let mut answer_memo = HashMap::new();
            for stone_txt in txt.split(" ") {
                let stone = stone_txt.parse::<u128>().unwrap();
                let (partial_total, temp_memo) = blink_stone_long(stone, 25, answer_memo);
                answer_memo = temp_memo;
                total += partial_total;
            }
            assert_eq!(total, 0);
        }
    }
}
