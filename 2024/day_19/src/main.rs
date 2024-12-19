use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(mut file_iter) = read_lines("input_19.txt") {
        let (towels, patterns) = process_input(&mut file_iter);
        println!(
            "part 1: {} in {:?}",
            find_reachable_patterns(&towels, &patterns),
            now.elapsed()
        );
        let mut found_matches: HashMap<String, usize> = HashMap::new();
        let weaves: Vec<Weave> = towels.iter().map(|t| Weave { pat: t.clone() }).collect();
        let mut total = 0;
        for pattern in patterns {
            total += re_find_poss_patterns(&weaves, &pattern, &mut found_matches);
        }
        println!("Part 2: {} in {:?}", total, now.elapsed());
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

fn process_input(i_file: &mut io::Lines<io::BufReader<File>>) -> (Vec<String>, Vec<String>) {
    let towels: Vec<String> = i_file
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    _ = i_file.next();
    let mut patterns: Vec<String> = Vec::new();
    for pattern in i_file {
        patterns.push(pattern.unwrap());
    }
    return (towels, patterns);
}

fn re_find_poss_patterns(
    towels: &Vec<Weave>,
    pattern: &String,
    confirmed_matches: &mut HashMap<String, usize>,
) -> usize {
    let mut reachable_patterns = 0;
    for weave in towels {
        if weave.pat == *pattern {
            reachable_patterns += 1;
        } else if pattern.starts_with(&weave.pat) {
            let stripped_pat = pattern.strip_prefix(&weave.pat).unwrap().to_owned();
            match confirmed_matches.get(&stripped_pat) {
                Some(u) => reachable_patterns += u,
                None => {
                    let tmp = re_find_poss_patterns(towels, &stripped_pat, confirmed_matches);
                    confirmed_matches.insert(stripped_pat, tmp);
                    reachable_patterns += tmp;
                }
            }
        }
    }
    return reachable_patterns;
}

fn find_reachable_patterns(towels: &Vec<String>, patterns: &Vec<String>) -> usize {
    let mut reachable_patterns = 0;
    let base_patterns: Vec<Weave> = towels.iter().map(|t| Weave { pat: t.clone() }).collect();
    'eachpattern: for pattern in patterns {
        let mut frontier = base_patterns.clone();
        loop {
            match frontier.pop() {
                Some(weave) => {
                    if weave.pat == *pattern {
                        reachable_patterns += 1;
                        continue 'eachpattern;
                    }

                    if pattern.starts_with(&weave.pat) {
                        let mut children = weave.create_children(&towels);
                        frontier.append(&mut children);
                    }
                }
                None => continue 'eachpattern,
            }
        }
    }
    reachable_patterns
}

#[derive(Clone)]
struct Weave {
    pat: String,
}

impl Weave {
    fn build(&mut self, pattern: &String) {
        self.pat.push_str(pattern);
    }

    fn create_children(self, patternset: &Vec<String>) -> Vec<Weave> {
        let mut children: Vec<Weave> = Vec::new();
        for pattern in patternset {
            let mut tmp = self.clone();
            tmp.build(&pattern);
            children.push(tmp);
        }
        return children;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let (towels, patterns) = process_input(&mut file_iter);
            assert_eq!(find_reachable_patterns(&towels, &patterns), 6);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let (towels, patterns) = process_input(&mut file_iter);
            let mut found_matches: HashMap<String, usize> = HashMap::new();
            let weaves: Vec<Weave> = towels.iter().map(|t| Weave { pat: t.clone() }).collect();
            let mut total = 0;
            for pattern in patterns {
                total += re_find_poss_patterns(&weaves, &pattern, &mut found_matches);
            }
            assert_eq!(total, 16);
        }
    }
}
