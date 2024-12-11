use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(mut file_iter) = read_lines("input_11.txt") {
        let txt = file_iter.next().unwrap().unwrap();
        let mut total = 0;
        for stone_txt in txt.split(" ") {
            let stone = stone_txt.parse::<u128>().unwrap();
            total += blink_stone_short(stone, 0);
        }
        println!("total 1: {}", total);
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

fn blink_stone_long(stone: u128, depth: usize, max_depth: usize, answer_memo: &HashMap<(u128, usize), usize>) -> (usize, HashMap<(u128, usize), usize>) {
    if depth == max_depth {
        let mut tmp_memo = answer_memo.clone();
        tmp_memo.insert((stone, depth), 1);
        return (1, tmp_memo);
    } else {
        let mut tmp_memo = answer_memo.clone();
        if stone == 0 {
            let poss = match answer_memo.get(&(1, depth + 1)) {
                Some(b) => b.to_owned(),
                None => blink_stone_long(1, depth + 1, max_depth, answer_memo).0,
            };
            tmp_memo.insert((stone, depth), poss);
            return (poss, tmp_memo);
        }
        let num_digits: u32 = (stone.ilog10() + 1).try_into().unwrap();
        if num_digits % 2 == 0 {
            let poss_1 = match answer_memo.get(&(stone / (10_u128.pow(num_digits / 2)), depth + 1)) {
                Some(a) => a.to_owned(),
                None => blink_stone_long(stone / (10_u128.pow(num_digits / 2)), depth + 1, max_depth, answer_memo).0,
            };
            let poss_2 = match answer_memo.get(&(stone % (10_u128.pow(num_digits / 2)), depth + 1)) {
                Some(b) => b.to_owned(),
                None => blink_stone_long(stone % (10_u128.pow(num_digits / 2)), depth + 1, max_depth, answer_memo).0,
            };
            tmp_memo.insert((stone, depth), poss_1 + poss_2);
            return (poss_1 + poss_2, tmp_memo);
        }
        let poss = match answer_memo.get(&(stone * 2024, depth + 1)) {
            Some(a) => a.to_owned(),
            None => blink_stone_long(stone * 2024, depth + 1, max_depth, answer_memo).0,
        };
        tmp_memo.insert((stone, depth), poss);
        return (poss, tmp_memo);
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
                let (partial_total, answer_memo) = blink_stone_long(stone, 0, 75, &answer_memo);
                total += partial_total;
            }
            assert_eq!(total, 0);
        }
    }
}
