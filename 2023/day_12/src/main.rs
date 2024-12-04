use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let mut total: u64 = 0;
    let re_numbers: regex::Regex = Regex::new(r"[0-9]+").unwrap();
    let re_row: regex::Regex = Regex::new(r"^[.?#]+").unwrap();
    let re_broken_groups: regex::Regex = Regex::new(r"[?#]+").unwrap();
    if let Ok(file_iter) = read_lines("input_12.txt") {
        for line in file_iter {
            if let Ok(text) = line {
                let mut cache: HashMap<(Vec<(usize, usize)>, Vec<usize>), usize> = HashMap::new();
                let broken_groups: Vec<usize> = re_numbers.find_iter(&text).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

                let current_row = re_row.find(&text).unwrap().as_str().chars().collect::<Vec<char>>();
                let mut canvas = vec!('.');
                let mut extended_broken_groups = Vec::new();
                for _ in 0..4 {
                    canvas.extend(current_row.clone());
                    canvas.extend(vec!('?'));
                    extended_broken_groups.extend(broken_groups.clone());
                }
                canvas.extend(current_row.clone());
                extended_broken_groups.extend(broken_groups.clone());
                canvas.extend(vec!('.'));
                /*
                let mut canvas2 = vec!('.');
                canvas2.push('?');
                canvas2.extend(current_row.clone());
                canvas2.extend(['?','.'].iter());
                let possible_groups_2: Vec<(usize, usize)> = re_broken_groups.find_iter(&(canvas2.iter().collect::<String>())).map(|m| (m.start(), m.len())).collect();
                let use_len = canvas2.len();
                canvas2[0] = canvas2[use_len-3];
                canvas2[use_len-1] = canvas2[2];
                let new_combinations = get_combinations2(&mut canvas2, &possible_groups_2, &broken_groups);
                println!("New method: {:?}", new_combinations);
                */
            
                let all_possible_groups: Vec<(usize, usize)> = re_broken_groups.find_iter(&(canvas.iter().collect::<String>())).map(|m| (m.start(), m.len())).collect();
                let all_combinations = get_combinations(&mut canvas, &all_possible_groups, &extended_broken_groups, &mut cache);
                total += all_combinations as u64;
                //println!("{}", total);
                //break;
                
            }
        }
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn get_combinations(canvas: &mut Vec<char>, remaining_possible_groups: &Vec<(usize, usize)>, remaining_groups_to_place: &Vec<usize>, cache: &mut HashMap<(Vec<(usize, usize)>, Vec<usize>), usize>) -> usize {
    
    match cache.get(&(remaining_possible_groups.clone(), remaining_groups_to_place.clone())).map(|entr| entr.clone()) {
        Some(res) => {
            return res;},
        None => {
            if remaining_groups_to_place.len() == 0 {
                for cha in 0..canvas.len() {
                    if canvas[cha] == '#' {
                        return 0;
                    }
                }
                return 1;
            }
            let mut results = 0;
            if remaining_groups_to_place.iter().sum::<usize>() <= remaining_possible_groups.iter().map(|m| m.1).sum() {
                let next_possible_group = remaining_possible_groups[0];
                let next_group_to_place = remaining_groups_to_place[0];
                let under_consideration = canvas[next_possible_group.0..next_possible_group.0 + next_possible_group.1].to_vec();
        
                if next_possible_group.1 >= next_group_to_place && canvas[next_possible_group.0 + next_group_to_place] != '#' && canvas[next_possible_group.0 - 1] != '#' {
                    
                    for i in 0..next_group_to_place {
                        canvas[next_possible_group.0 + i] = 'P';
                    }
                    canvas[next_possible_group.0 + next_group_to_place] = '.';
                    
                    let mut new_remaining_possible = remaining_possible_groups[1..remaining_possible_groups.len()].to_vec();
                    let new_remaining_groups = remaining_groups_to_place[1..remaining_groups_to_place.len()].to_vec();
                    if next_possible_group.1 > next_group_to_place + 1 {
                        new_remaining_possible.insert(0,(next_possible_group.0+next_group_to_place+1, next_possible_group.1-next_group_to_place-1));
                    }
                    let with_broken = get_combinations(canvas, &new_remaining_possible, &new_remaining_groups, cache);
                    results += with_broken;
                } 
                for i in 0..next_possible_group.1 {
                    canvas[next_possible_group.0 + i] = under_consideration[i];
                }
                if canvas[next_possible_group.0] != '#' {
                    let mut new_remaining_possible = Vec::new();
                    if next_possible_group.1 != 1 {
                        new_remaining_possible.push((next_possible_group.0 + 1, next_possible_group.1 - 1));
                    }
                    new_remaining_possible.extend(&remaining_possible_groups[1..remaining_possible_groups.len()]);
                    let without_broken = get_combinations(canvas, &new_remaining_possible, remaining_groups_to_place, cache);
                    results += without_broken;
                }
            }
            cache.insert((remaining_possible_groups.clone(), remaining_groups_to_place.clone()), results);
            results
        }
    }
}
    
/*
fn used_railing(canvas: &Vec<char>) -> Option<(usize, usize)> {
    if canvas[1] == 'P' {
        return Some((1,0));
    } 
    if canvas[canvas.len()-2] == 'P' {
        return Some((0,1));
    }
    None
}

fn get_big_combos(combinis: (usize, usize, usize)) -> usize {
    if combinis.1 == 0 || combinis.2 == 0 {
        let mut result = combinis.0 + combinis.2;
        for _ in 0..3 {
            result *= combinis.0 + combinis.1 + combinis.2;
        }
        result *= combinis.0 + combinis.1;
        return result;
    }
    return combinis_score(combinis, 0, &(Vec::new()), 0);
}

fn combinis_score(combinis: (usize, usize, usize), depth: usize, current_path: &Vec<char>, current_score: usize) -> usize {
    let mut new_score = 0;
    if depth == 5 {
        //println!("{:?}", current_path);
        //println!("Score: {}, combinis: {:?}", current_score, combinis);
        return current_score;
    }
    let mut path = current_path.clone();
    if depth == 0 {
        if combinis.2 != 0 {
            path.push('R');
            new_score += combinis_score(combinis, depth + 1, &path, combinis.2);
            path.pop();
        }
        path.push('C');
        new_score += combinis_score(combinis, depth + 1, &path, combinis.0);
    } else if depth == 4 {
        if path[3] != 'R' && combinis.1 != 0 {
            path.push('L');
            new_score += combinis_score(combinis, depth + 1, &path, current_score * combinis.1);
            path.pop();
        }
        path.push('C');
        new_score += combinis_score(combinis, depth + 1, &path, current_score * combinis.0);
    } else {
        if path[depth-1] != 'R' && combinis.1 != 0{
            path.push('L');
            new_score += combinis_score(combinis, depth + 1, &path, current_score * combinis.1);
            path.pop();
        }
        path.push('C');
        new_score += combinis_score(combinis, depth + 1, &path, current_score * combinis.0);
        path.pop();
        if combinis.2 != 0 {
            path.push('R');
            new_score += combinis_score(combinis, depth + 1, &path, current_score * combinis.2);
        }
    }
    new_score
}

fn get_combinations2(canvas: &mut Vec<char>, remaining_possible_groups: &Vec<(usize, usize)>, remaining_groups_to_place: &Vec<usize>) -> (usize, usize, usize) {
    if remaining_groups_to_place.len() == 0 {
        //println!("Successful canvas: {:?}", canvas);
        let tmp = used_railing(canvas);
        if tmp.is_none() { return (1,0,0); }
        let a = tmp.unwrap();
        return (0, a.0, a.1);
    }
    let mut results = (0, 0, 0);
    if remaining_groups_to_place.iter().sum::<usize>() < remaining_possible_groups.iter().map(|m| m.1).sum::<usize>() + 1 {
        let next_possible_group = remaining_possible_groups[0];
        let next_group_to_place = remaining_groups_to_place[0];
        let under_consideration = canvas[next_possible_group.0..next_possible_group.0 + next_possible_group.1].to_vec();
    
        if next_possible_group.1 >= next_group_to_place && canvas[next_possible_group.0 + next_group_to_place] != '#' && canvas[next_possible_group.0 - 1] !='#' {
            
            for i in 0..next_group_to_place {
                canvas[next_possible_group.0 + i] = 'P';
            }
            canvas[next_possible_group.0 + next_group_to_place] = '.';
            
            let mut new_remaining_possible = remaining_possible_groups[1..remaining_possible_groups.len()].to_vec();
            let new_remaining_groups = remaining_groups_to_place[1..remaining_groups_to_place.len()].to_vec();
            if next_possible_group.1 > next_group_to_place + 1 {
                new_remaining_possible.insert(0,(next_possible_group.0+next_group_to_place+1, next_possible_group.1-next_group_to_place-1));
            }
            let with_broken = get_combinations2(canvas, &new_remaining_possible, &new_remaining_groups);
            results = (results.0 + with_broken.0, results.1 + with_broken.1, results.2 + with_broken.2);
        } 
        for i in 0..next_possible_group.1 {
            canvas[next_possible_group.0 + i] = under_consideration[i];
        }
        //println!("Canvas before shift: {:?}", canvas);
        if canvas[next_possible_group.0] != '#' {
            let mut new_remaining_possible = Vec::new();
            if next_possible_group.1 != 1 {
                new_remaining_possible.push((next_possible_group.0 + 1, next_possible_group.1 - 1));
            }
            new_remaining_possible.extend(&remaining_possible_groups[1..remaining_possible_groups.len()]);
            let without_broken = get_combinations2(canvas, &new_remaining_possible, remaining_groups_to_place);
            results = (results.0 + without_broken.0, results.1 + without_broken.1, results.2 + without_broken.2);
        }
    }
    results
}

fn matches_pattern(row_text: &Vec<char>, broken_groups: &Vec<usize>, re_broken_groups: &regex::Regex) -> bool {
    let row_str = row_text.into_iter().collect::<String>();
    //println!("Evaluating {}", row_str);
    let broken_group_nums = (*re_broken_groups).find_iter(&row_str).map(|m| m.as_str().len()).collect::<Vec<usize>>();
    //println!("Found broken groups {:?}, compared to {:?} target", broken_group_nums, *broken_groups);
    if  broken_group_nums == *broken_groups {
        //println!("Match!");
        return true;
    }
    //println!("not match");
    return false;
}

fn factorial(num: usize) -> usize {
    (1..=num).product()
}
*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

