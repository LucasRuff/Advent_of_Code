use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_23.txt") {
        let all_input: String = file_iter.map(|line| line.unwrap()).collect::<Vec<String>>().join(":");
        let re_comp: Regex = Regex::new(r"([[:lower:]]+)\-([[:lower:]]+)").unwrap();
        let mut t_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
        let mut all_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
        let mut t_lans: HashSet<Vec<usize>> = HashSet::new();
        for (_, [comp_a, comp_b]) in re_comp.captures_iter(&all_input).map(|c| c.extract()) {
            let comp_a_chars: Vec<char> = comp_a.chars().take(2).collect();
            let comp_b_chars: Vec<char> = comp_b.chars().take(2).collect();
            let comp_a_map: usize = map_comp((comp_a_chars[0], comp_a_chars[1]));
            let comp_b_map: usize = map_comp((comp_b_chars[0], comp_b_chars[1]));
            if comp_a_chars[0] == 't' || comp_b_chars[0] == 't' {
                t_matches[comp_a_map][comp_b_map] = 1;
                t_matches[comp_b_map][comp_a_map] = 1;
            }
            all_matches[comp_a_map][comp_b_map] = 1;
            all_matches[comp_b_map][comp_a_map] = 1;
        }
        for t_row in map_comp(('t','a'))..map_comp(('u','a')) {
            for i in 0..26*26 {
                if t_matches[t_row][i] == 1 {
                    for j in 0..i {
                        if t_matches[t_row][j] == 1 && all_matches[i][j] == 1 && i != t_row && j != t_row {
                            let mut lan_party = vec![t_row, i, j];
                            lan_party.sort();
                            t_lans.insert(lan_party);
                        }
                    }
                }
            }
        }
        println!("Part 1: {}", t_lans.len());
        
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

fn map_comp(name: (char, char)) -> usize {
    let chars = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    return 26 * chars.binary_search(&name.0).unwrap() + chars.binary_search(&name.1).unwrap();
}

fn bronkerbosh(r: HashSet<usize>, p: HashSet<usize>, x: HashSet<usize>, adj: &Vec<Vec<usize>>, degs: &Vec<(usize, usize)>, cliques: &mut Vec<HashSet<usize>>) {
    
    if p.is_empty() && x.is_empty() {cliques.push(r.clone());} else {
        let mut local_r = r.clone();
        let mut local_p = p.clone();
        let mut local_x = x.clone();
        let mut current_pivot = (0,0);
        for (potential_pivot, deg) in degs {
            if *deg > current_pivot.1 && p.contains(potential_pivot) {
                current_pivot = (*potential_pivot, *deg);
            }
        }
        let mut neighbors: HashSet<usize> = HashSet::new();
        for (i,item) in adj[current_pivot.0].iter().enumerate() {
            if *item >= 1 {neighbors.insert(i);}
        }
        let outer_loop = p.difference(&neighbors);
        for vertex in outer_loop {
            let new_r: HashSet<_> = local_r.union(&HashSet::from([*vertex])).map(|e| *e).collect();
            let new_p: HashSet<_> = local_p.intersection(&neighbors).map(|e| *e).collect();
            let new_x: HashSet<_> = local_x.intersection(&neighbors).map(|e| *e).collect();
            bronkerbosh(new_r, new_p, new_x, adj, degs, cliques);
            _ = local_p.remove(&vertex);
            local_x = local_x.union(&HashSet::from([*vertex])).map(|e| *e).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let all_input: String = file_iter.map(|line| line.unwrap()).collect::<Vec<String>>().join(":");
            let re_comp: Regex = Regex::new(r"([[:lower:]]+)\-([[:lower:]]+)").unwrap();
            let mut t_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
            let mut all_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
            let mut t_lans: HashSet<Vec<usize>> = HashSet::new();
            let mut total = 0;
            for (_, [comp_a, comp_b]) in re_comp.captures_iter(&all_input).map(|c| c.extract()) {
                let comp_a_chars: Vec<char> = comp_a.chars().take(2).collect();
                let comp_b_chars: Vec<char> = comp_b.chars().take(2).collect();
                let comp_a_map: usize = map_comp((comp_a_chars[0], comp_a_chars[1]));
                let comp_b_map: usize = map_comp((comp_b_chars[0], comp_b_chars[1]));
                if comp_a_chars[0] == 't' || comp_b_chars[0] == 't' {
                    t_matches[comp_a_map][comp_b_map] = 1;
                    t_matches[comp_b_map][comp_a_map] = 1;
                }
                all_matches[comp_a_map][comp_b_map] = 1;
                all_matches[comp_b_map][comp_a_map] = 1;
            }
            assert_eq!(1, all_matches[map_comp(('w','h'))][map_comp(('y','n'))]);
            for t_row in map_comp(('t','a'))..map_comp(('u','a')) {
                for i in 0..26*26 {
                    if t_matches[t_row][i] == 1 {
                        for j in 0..i {
                            if t_matches[t_row][j] == 1 && all_matches[i][j] == 1 && i != t_row && j != t_row {
                                let mut lan_party = vec![t_row, i, j];
                                lan_party.sort();
                                t_lans.insert(lan_party);
                            }
                        }
                    }
                }
            }
            assert_eq!(7, t_lans.len());
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let all_input: String = file_iter.map(|line| line.unwrap()).collect::<Vec<String>>().join(":");
            let re_comp: Regex = Regex::new(r"([[:lower:]]+)\-([[:lower:]]+)").unwrap();
            let mut t_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
            let mut all_matches: Vec<Vec<usize>> = vec![vec![0;26*26];26*26];
            let mut t_lans: HashSet<Vec<usize>> = HashSet::new();
            let mut total = 0;
            for (_, [comp_a, comp_b]) in re_comp.captures_iter(&all_input).map(|c| c.extract()) {
                let comp_a_chars: Vec<char> = comp_a.chars().take(2).collect();
                let comp_b_chars: Vec<char> = comp_b.chars().take(2).collect();
                let comp_a_map: usize = map_comp((comp_a_chars[0], comp_a_chars[1]));
                let comp_b_map: usize = map_comp((comp_b_chars[0], comp_b_chars[1]));
                if comp_a_chars[0] == 't' || comp_b_chars[0] == 't' {
                    t_matches[comp_a_map][comp_b_map] = 1;
                    t_matches[comp_b_map][comp_a_map] = 1;
                }
                all_matches[comp_a_map][comp_b_map] = 1;
                all_matches[comp_b_map][comp_a_map] = 1;
            }
            let mut cliques: Vec<HashSet<usize>> = Vec::new();
            let mut degs : Vec<(usize, usize)> = Vec::new();
            let mut initial_p = HashSet::new();
            for (i,row) in all_matches.iter().enumerate() {
                let mut k = 0;
                for col in row {
                    k += col;
                }
                if k > 0 {
                    initial_p.insert(i);
                }
                degs.push((i, k));
            }
            
            bronkerbosh(HashSet::<usize>::new(), initial_p, HashSet::<usize>::new(), &all_matches, &degs, &mut cliques);
            println!("{:?}", cliques);
            panic!();
        }
    }
}
