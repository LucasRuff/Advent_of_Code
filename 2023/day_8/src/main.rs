use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let re_line: regex::Regex = Regex::new(r"[0-9A-Z][0-9A-Z][0-9A-Z]").unwrap();
    let re_end = Regex::new(r"[Z]$").unwrap();
    let re_start = Regex::new(r"[A]$").unwrap();
    let now = Instant::now();
    if let Ok(mut file_iter) = read_lines("input_8.txt") {
        let mut total :u64 = 1000;
        let turns: Vec<char> = file_iter.next().unwrap().unwrap().chars().collect();
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let mut current_nodes: Vec<String> = Vec::new();
        let _ = file_iter.next();
        for line in file_iter {
            if let Ok(text) = line {
                let mut parsed_text = re_line.find_iter(&text);
                let node_name = parsed_text.next().unwrap().as_str().to_owned();
                let node_left = parsed_text.next().unwrap().as_str().to_owned();
                let node_right = parsed_text.next().unwrap().as_str().to_owned();
                if re_start.is_match(&(node_name)) {
                    current_nodes.push(node_name.clone());
                }
                nodes.insert(node_name.clone(), Node{ name: node_name, left_name: node_left, right_name: node_right});
            }
        }

        //let mut current_node = find_start(nodes.clone()).unwrap();
        let mut direction_num = 0;
        for start_node in current_nodes {
            'bigloop: loop {
                let mut num_matches = 0;
                //let mut next_node_names = Vec::new();
                //println!("Current Nodes: {:?}", current_nodes);
                for node in current_nodes.clone() {
                    if re_end.is_match(&node) {
                        num_matches += 1;
                    }
                }
                if num_matches == current_nodes.len() { break 'bigloop };            
    
                current_nodes = take_step(current_nodes, &nodes, turns[direction_num]);
    
                direction_num += 1;
                total -= 1;
                if direction_num == turns.len() {
                    direction_num = 0;
                }
                if total == 0 {
                    println!("Current nodes: {:?}", current_nodes);
                    break 'bigloop;
                }
            }
        }
        
        println!("time: {:?}", now.elapsed());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    name: String,
    left_name: String,
    right_name: String,
}
/*
fn find_start (node_list: Vec<Node>) -> Option<Node> {
    for node in node_list {
        if node.name == "AAA" {
            return Some(node.clone());
        }
    }
    return None;
}
*/
fn take_step (node_names: Vec<String>, node_list: &HashMap<String, Node>, direction: char) -> Vec<String> {
    let mut result = Vec::new();
    for node_name in node_names {
        if direction == 'L' {
            result.push(node_list.get(node_name.as_str()).unwrap().left_name.clone());
        } else {
            result.push(node_list.get(node_name.as_str()).unwrap().right_name.clone());
        }
    }
    return result;
}