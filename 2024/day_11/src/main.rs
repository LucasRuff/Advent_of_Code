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
            let (res_total, res_memo) = blink_stone_long(stone, 75, blink_memo);
            blink_memo = res_memo;
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

fn blink_stone_tree() {}

/*
fn blink_stone_long(parent_stone: u128, max_depth: usize, start_memo: HashMap<u128, (usize, (u128, u128))>) -> (usize, HashMap<u128, (usize, (u128, u128))>) {
    let mut memo = start_memo.clone();
    let mut frontier = vec![Stone{val: parent_stone, depth: 0}];
    let mut total = 0;
    'bigloop: loop {
        let current_stone = frontier.pop();
        match current_stone {
            Some(s) => {
                match memo.get(&s.val) {
                    Some((steps, (stone_1_val, stone_2_val))) => {
                        let moving_depth = s.depth + steps;
                        if moving_depth <= max_depth {
                            let (stone_1, stone_2) = (Stone{val: *stone_1_val, depth: moving_depth},
                            Stone{val: *stone_2_val, depth: moving_depth});
                            frontier.push(stone_1);
                            frontier.push(stone_2);
                            continue 'bigloop;
                        } else {
                            total += 1;
                            continue 'bigloop;
                        }
                    }
                    None => {
                        println!("memo miss on {}", s.val);
                    },
                }
                if s.depth == max_depth {
                    total += 1;
                    continue 'bigloop;
                }
                let mut num_digits: u32 = if s.val != 0 {(s.val.ilog10() + 1).try_into().unwrap()} else {1};
                let mut moving_val = s.val;
                let mut moving_depth = s.depth;
                'small_loop: loop {
                    
                    match (num_digits % 2 == 0, moving_val == 0) {
                        (false,true) => {
                            moving_val = 1;
                            moving_depth += 1;
                            
                        },
                        (true,_) => {
                            moving_depth += 1;
                            
                            let (stone_1, stone_2) = (Stone{val: moving_val / 10_u128.pow(num_digits / 2), depth: moving_depth},
                            Stone{val: moving_val % (10_u128.pow(num_digits / 2)), depth: moving_depth});
                            memo.insert(s.val, (moving_depth - s.depth, (stone_1.val, stone_2.val)));
                            println!("New memo entry for {}", moving_val);
                            if moving_depth == max_depth {
                                total += 2;
                                continue 'bigloop;
                            }
                            frontier.push(stone_1);
                            frontier.push(stone_2);
                            continue 'bigloop;

                        },
                        (false,false) => {
                            moving_val = moving_val * 2024;
                            moving_depth += 1;
                        },
                    }
                    if moving_depth == max_depth {
                        total += 1;
                        continue 'bigloop;
                    }
                    num_digits = if moving_val != 0 {(moving_val.ilog10() + 1).try_into().unwrap()} else {1};
                }
            },
            None => break 'bigloop,
        }
    }
    return (total, memo);
}
*/

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

#[derive(Debug, Clone, PartialEq)]
struct Stone {
    val: u128,
    depth: usize,
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
