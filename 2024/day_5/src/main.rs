use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_5.txt") {
    let (rule_list, page_list) = process_input(file_iter);
    let rules = get_rules(rule_list);
        let mut total1 = 0;
        let mut total2 = 0;
        for page in page_list {
            if passes_rules(&page, &rules).0 {
                total1 += get_middle(page);
            } else {
                total2 += get_middle(make_pass_rules(&page, &rules));
            }
        }
        println!("Total 1: {}", total1);
        println!("Total 2: {}", total2);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn process_input(i_lines: io::Lines<io::BufReader<File>>) -> (Vec::<String>, Vec::<Vec::<usize>>) {
    let mut rule_list: Vec::<String> = Vec::new();
    let mut page_list: Vec::<Vec<usize>> = Vec::new();
    let mut rules = true;
    for line in i_lines {
        let text = line.unwrap();
        if text == "" {
            rules = false;
            continue;
        }
        if rules {
            rule_list.push(text);
        } else {
            page_list.push(text.split(",").map(|num| num.parse::<usize>().unwrap()).collect());
        }
    }
    return (rule_list, page_list);
}

fn get_rules(rule_list: Vec::<String>) -> HashMap::<usize, Vec<usize>> {
    let mut new_rules = HashMap::<usize, Vec<usize>>::new();
    for rule in rule_list {
        let mut rule_chars = rule.chars();
        let start = (vec![rule_chars.next().unwrap(),rule_chars.next().unwrap()]).into_iter().collect::<String>().parse::<usize>().unwrap();
        _ = rule_chars.next();
        let end = (vec![rule_chars.next().unwrap(),rule_chars.next().unwrap()]).into_iter().collect::<String>().parse::<usize>().unwrap();
        new_rules.entry(start).and_modify(|afters| afters.push(end)).or_insert(vec![end]);
    }
    
    return new_rules;
}

fn passes_rules(to_print: &Vec::<usize>, rules: &HashMap::<usize, Vec<usize>>) -> (bool, usize, usize) {
    for (i,num) in to_print.iter().enumerate() {
        if rules.get(num).is_some() {
            for j in 0..i {
                match rules.get(num).expect("the rule disappeared?").contains(&to_print[j]) {
                    true => return (false, i, j),
                    false => continue,
                }
            }
        }
    }
    return (true, 0, 0);
}

fn make_pass_rules(to_print: &Vec::<usize>, rules: &HashMap::<usize, Vec<usize>>) -> Vec::<usize> {
    let mut formed_str: Vec::<usize> = to_print.clone();
    let (mut pass, mut swap1, mut swap2) = passes_rules(&formed_str, &rules);
    while !pass {
        let tmp = formed_str[swap1];
        formed_str[swap1] = formed_str[swap2];
        formed_str[swap2] = tmp;
        (pass, swap1, swap2) = passes_rules(&formed_str, &rules);
    }
    return formed_str;
}

fn get_middle(i_str: Vec::<usize>) -> usize {
    let len = i_str.len();
    return i_str[(len-1)/2];
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
            let (rule_list, page_list) = process_input(file_iter);
            let rules = get_rules(rule_list);
            let mut total = 0;
            for page in page_list {
                if passes_rules(&page, &rules).0 {
                    total += get_middle(page);
                }
            }
            assert_eq!(total, 143);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (rule_list, page_list) = process_input(file_iter);
            let rules = get_rules(rule_list);
            let mut total = 0;
            for page in page_list {
                if ! passes_rules(&page, &rules).0 {
                    total += get_middle(make_pass_rules(&page, &rules));
                }
            }
            assert_eq!(total, 123);
        }
    }

    #[test]
    fn test_get_middle() {
        let test_str = vec![75,47,61,53,29];
        assert_eq!(get_middle(test_str), 61);
    }

    #[test]
    fn test_passes_rules() {
        let test_strs = vec![vec![75,47,61,53,29], vec![61,13,29]];
        let rules = HashMap::from([
            (29, vec![13]),
            (75, vec![61]),
        ]);
        assert_eq!((passes_rules(&test_strs[0],&rules).0,passes_rules(&test_strs[1],&rules).0),(true,false));
    }
}