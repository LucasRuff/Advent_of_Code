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
            let (res_total, res_memo) = blink_stone_recurse(stone, 0, 75, blink_memo, 0);
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

fn check_memo(parent_stone: &Stone, max_depth: usize, memo: &HashMap<u128, StoneFacts>) -> (usize, Option<Vec<Stone>>) {
    match memo.get(&parent_stone.val) {
        Some(parent_facts) => {
            // we've seen this stone before
            let parent_splits_at = parent_stone.depth + parent_facts.splitting_depth_fact;
            if parent_splits_at <= max_depth {
                // it's shallow enough that we can't jump to the end
                
                /*
                let mut num_children_buffer = 2; // need this to store previous value of children, so we can go one step back on our splits
                for (split_step, _, children_after_splits) in facts.known_splits.clone() {
                    if parent_stone.depth + split_step >= max_depth { // can't split again before max depth
                        return (num_children_buffer, None);
                    }
                    // we can make another split before reaching max depth
                    num_children_buffer = children_after_splits;
                }// we've never seen this many splits before

                // copy over the children's split buffer
                let mut new_splits = facts.known_splits.clone();
                for i in 0..1 {
                    let child_facts = memo.get(&facts.children_fact[i]).unwrap();
                    for child_split in child_facts.known_splits.clone() {
                        for j in 0..new_splits.len() {
                            if child_split.0 == new_splits[j].0 {
                                match (i, new_splits[j].1) {
                                    (0,Left) => {},
                                    (0,Right) => {
                                        new_splits[j].1 = Branches::Both;
                                        new_splits[j].2 += child_split.2;
                                    },
                                    (1,Right) => {
                                        new_splits[]
                                    }
                                }
                            }
                        }
                        new_splits.push((child_split.0 + facts.split_depth), child_split.2 + 1, child_split + 1);
                    }
                    
                }
                new_facts.known_splits.sort_by(|a, b| a.cmp(b));
                */
                // test if we can get there by doing one jump from children
                let child_a_skip_option = memo.get(&parent_facts.children_fact[0]);
                let child_b_skip_option = memo.get(&parent_facts.children_fact[1]);
                match (child_a_skip_option, child_b_skip_option) {
                    (Some(facts_a), Some(facts_b)) => { // have memoized split data on both children
                        let child_a_splits_at = facts_a.splitting_depth_fact;
                        let child_b_splits_at = facts_b.splitting_depth_fact;
                        match (parent_splits_at + child_a_splits_at > max_depth, parent_splits_at + child_b_splits_at > max_depth) {
                            (true, true) => return (2, None),
                            (true, false) => {
                                return (1, Some(vec![Stone{val: parent_facts.children_fact[1], depth: parent_splits_at}]));
                            },
                            (false, true) => {
                                return (1, Some(vec![Stone{val: parent_facts.children_fact[0], depth: parent_splits_at}]));
                            },
                            (false, false) => {
                                return (0, Some(vec![
                                    Stone{val: parent_facts.children_fact[0], depth: parent_splits_at},
                                    Stone{val: parent_facts.children_fact[1], depth: parent_splits_at},
                                ]));
                            },
                        }
                    },
                    (Some(child_facts), None) => { // seen just child A child before
                        let child_a_splits_at = child_facts.splitting_depth_fact;
                        if child_a_splits_at + parent_splits_at > max_depth {
                            return (1, Some(vec![Stone{val: parent_facts.children_fact[1], depth: parent_splits_at}]));
                        } else {
                            return (0, Some(vec![
                                Stone{val: parent_facts.children_fact[0], depth: parent_splits_at},
                                Stone{val: parent_facts.children_fact[1], depth: parent_splits_at},
                            ]));
                        }
                    },
                    (None, Some(child_facts)) => { // seen just child B before
                        let child_b_splits_at = child_facts.splitting_depth_fact;
                        if child_b_splits_at + parent_splits_at > max_depth {
                            return (1, Some(vec![Stone{val: parent_facts.children_fact[0], depth: parent_splits_at}]));
                        } else {
                            return (0, Some(vec![
                                Stone{val: parent_facts.children_fact[0], depth: parent_splits_at},
                                Stone{val: parent_facts.children_fact[1], depth: parent_splits_at},
                            ]));
                        }
                    },
                    (None, None) => { // never seen either child before
                        return (0, Some(vec![
                            Stone{val: parent_facts.children_fact[0], depth: parent_splits_at},
                            Stone{val: parent_facts.children_fact[1], depth: parent_splits_at},
                        ]));
                    }
                }
                
            } else {
                // it's too deep to split
                return (1, None);
            }
        },
        None => {
            // never seen this stone before
            return (0, None);
        }
    }
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

fn blink_stone_recurse(parent_stone: u128, depth: usize, max_depth: usize, start_memo: HashMap<(u128, usize), usize>, iter_depth: usize) -> (usize, HashMap<(u128, usize), usize>) {
    if depth == max_depth {
        return (1, start_memo);
    }
    println!("Iteration depth: {}", iter_depth);
    match start_memo.get(&(parent_stone, depth)) {
        Some(score) => {
            return (*score, start_memo);
        },
        None => {
            let mut new_memo = start_memo.clone();
            let (split_depth, children) = find_first_split(parent_stone, max_depth);
            if split_depth + depth > max_depth {
                new_memo.insert((parent_stone, depth),1);
                return (1, new_memo);
            }
            let mut total = 0;
            if children.is_some() {
                for child in children.unwrap().iter() {
                    let (child_score, new_memo) = blink_stone_recurse(*child, split_depth + depth, max_depth, new_memo.clone(), iter_depth + 1);
                    total += child_score;
                }
                new_memo.insert((parent_stone, depth),total);
                return (total, new_memo);
            }
            new_memo.insert((parent_stone, depth), 1);
            return (1, new_memo);
        },
    }
}


fn blink_stone_long(parent_stone: u128, max_depth: usize, start_memo: HashMap<u128, StoneFacts>) -> (usize, HashMap<u128, StoneFacts>) {
    let mut memo = start_memo.clone();
    let mut frontier = vec![Stone{val: parent_stone, depth: 0}];
    let mut total = 0;
    'bigloop: loop {
        //println!("Current frontier: {:?}", frontier);
        let current_stone = frontier.pop();
        //println!("Evaluating stone {:?}", current_stone);
        match current_stone {
            Some(s) => {
                if s.depth == max_depth {
                    total += 1;
                    //println!("It was at max depth");
                    continue 'bigloop;
                }
                //let (total_to_add, new_stones) = check_memo(&s, max_depth, &memo, 5, 0);
                match check_memo(&s, max_depth, &memo) {
                    (return_children, Some(to_queue)) => {
                        //println!("Stone was in memo, with {} known children before finish", return_children);
                        total += return_children;
                        for new_stone in to_queue {
                            frontier.push(new_stone);
                        }
                        continue 'bigloop;
                    },
                    
                    (0, None) => { // not in memo
                        //println!("Stone not in memo");
                        let (split_depth, children) = find_first_split(s.val, max_depth);
                        if children.is_some() {
                            memo.insert(s.val, StoneFacts{splitting_depth_fact: split_depth, children_fact: children.clone().unwrap()});
                        }
                        if s.depth + split_depth > max_depth {
                            total += 1;
                            continue 'bigloop;
                        }
                        match children {
                            Some(child_vec) => {
                                for child in child_vec {
                                    frontier.push(Stone{val: child, depth: s.depth + split_depth});
                                }
                            },
                            None => {},
                        }
                    },
                    (ret_child_num, None) => { // too deep to split again
                        //println!("Split was in memo but too deep to split again");
                        total += ret_child_num;
                        continue 'bigloop;
                    },
                }
            },
            None => break 'bigloop,
        }
    }
    return (total, memo);
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

#[derive(Debug, Clone, PartialEq)]
struct StoneFacts{
    splitting_depth_fact: usize,
    children_fact: Vec<u128>,
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
