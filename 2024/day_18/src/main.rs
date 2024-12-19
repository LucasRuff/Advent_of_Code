use regex::Regex;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_18.txt") {
        let (initial_node, mut maze, remaining_rocks) = get_maze(file_iter, 70, 1024);
        println!("Maze retrieved: {:?}", now.elapsed());
        let (best_path, path_chain) = navigate_maze(initial_node.clone(), &maze, false).unwrap();
        println!("part 1: {:?} in {:?}", best_path, now.elapsed());
        println!("Part 2: {:?} ", find_latest_drop(initial_node, &mut maze, remaining_rocks, path_chain, false));
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_maze(i_file: io::Lines<io::BufReader<File>>, size: usize, num_bytes: usize) -> (Node, Maze, Vec<(usize, usize)>) {
    let mut maze: Vec<Vec<char>> = Vec::new();
    let push_vec: Vec<char> = vec!['.'; size + 1];
    for _ in 0..size+1 {
        maze.push(push_vec.clone());
    }
    let (goal_x, goal_y): (usize, usize) = (size, size);
    let all_input: String = i_file.map(|line| line.unwrap()).collect::<Vec<String>>().join(":");
    let re_number: Regex = Regex::new(r"([0-9]+)\,([0-9]+)").unwrap();
    let mut bytes_processed: usize = 0;
    let mut undropped_rocks: Vec<(usize, usize)> = Vec::new();
    for (_, [drop_x, drop_y]) in re_number.captures_iter(&all_input).map(|c| c.extract()) {
        
        let x_val = drop_x.parse::<usize>().unwrap();
        let y_val = drop_y.parse::<usize>().unwrap();
        if bytes_processed >= num_bytes {
            undropped_rocks.push((y_val, x_val));
            continue;
        }
        maze[y_val][x_val] = '#';
        bytes_processed += 1;
        
    }

    return (
        Node::origin((0, 0)),
        Maze {
            map: maze,
            goal: (goal_y, goal_x),
            limits: (size, size),
        },
        undropped_rocks
    );
}

fn navigate_maze(start_node: Node, maze: &Maze, debug: bool) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut visited_nodes: HashMap< /*location*/ (usize, usize), /*parent*/ (usize, usize)> =
        HashMap::new();
    let mut frontier: Vec<Node> = vec![start_node];
    loop {
        if let Some(current_node) = frontier.pop() {
            if debug {
                println!("visiting node {}, {}", current_node.p_y, current_node.p_x);
            }
            if (current_node.p_y, current_node.p_x) == maze.goal {
                let mut parent = current_node.parent;
                let mut chain: Vec<(usize, usize)> = Vec::new();
                while parent != (0,0) {
                    chain.push(parent);
                    parent = *visited_nodes.get(&parent).unwrap();
                }
                return Some((current_node.cost_to_go, chain));
            }
            let children = current_node.generate_children(&maze);
            for child in children {
                if visited_nodes.get(&(child.p_y, child.p_x)).is_none() {
                    visited_nodes.insert((child.p_y, child.p_x), (current_node.p_y, current_node.p_x));
                    frontier.push(child);
                }
            }
            frontier.sort();
            frontier.reverse();
            if debug {
                println!("Current frontier:\n{:?}", frontier);
            }
        } else {
            return None;
        }
    }
}

fn find_latest_drop(start_node: Node, maze: &mut Maze, remaining_rocks: Vec<(usize, usize)>, starter_chain: Vec<(usize, usize)>, debug: bool) -> (usize, usize) {
    let mut current_chain = starter_chain.clone();
    for rock in remaining_rocks {
        maze.add_byte(rock);
        if current_chain.contains(&rock) {
            match navigate_maze(start_node.clone(), &maze, debug) {
                Some((_, chain)) => {current_chain = chain},
                None => return rock,
            }
        } 
    }
    return (0,0);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug, Eq, Clone)]
struct Node {
    p_x: usize,
    p_y: usize,
    cost_to_go: usize,
    heuristic: usize,
    parent: (usize, usize),
}

impl Node {
    fn new(parent: &Node, direction: Direction, goal: (usize, usize)) -> Node {
        let px = parent.p_x + if direction == Direction::Right { 1 } else { 0 }
            - if direction == Direction::Left { 1 } else { 0 };
        let py = parent.p_y + if direction == Direction::Down { 1 } else { 0 }
            - if direction == Direction::Up { 1 } else { 0 };
        let cost = parent.cost_to_go + 1;
        let heur = cost + goal.1 - px + goal.0 - py;

        Node {
            p_x: px,
            p_y: py,
            cost_to_go: cost,
            heuristic: heur,
            parent: (parent.p_y, parent.p_x),
        }
    }

    fn origin(point: (usize, usize)) -> Node {
        Node {
            p_x: point.1,
            p_y: point.0,
            cost_to_go: 0,
            heuristic: 0,
            parent: (0,0),
        }
    }

    fn generate_children(&self, maze: &Maze) -> Vec<Node> {
        let mut child_vec: Vec<Node> = Vec::new();
        if self.p_x < maze.limits.1 && maze.map[self.p_y][self.p_x + 1] != '#' {
            child_vec.push(Node::new(&self, Direction::Right, maze.goal));
        }
        if self.p_x > 0 && maze.map[self.p_y][self.p_x - 1] != '#'  {
            child_vec.push(Node::new(&self, Direction::Left, maze.goal));
        }
        if self.p_y < maze.limits.1 && maze.map[self.p_y + 1][self.p_x] != '#' {
            child_vec.push(Node::new(&self, Direction::Down, maze.goal));
        }
        if self.p_y > 0 && maze.map[self.p_y - 1][self.p_x] != '#'  {
            child_vec.push(Node::new(&self, Direction::Up, maze.goal));
        }
        return child_vec;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        if self.heuristic == other.heuristic {
            return Equal;
        } else if self.heuristic >= other.heuristic {
            return Greater;
        } else if self.heuristic <= other.heuristic {
            return Less;
        } else {
            return Equal;
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        if self.heuristic == other.heuristic {
            return Some(Equal);
        } else if self.heuristic >= other.heuristic {
            return Some(Greater);
        } else if self.heuristic <= other.heuristic {
            return Some(Less);
        } else {
            return None;
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.p_y, self.p_x)
    }
}

struct Maze {
    map: Vec<Vec<char>>,
    goal: (usize, usize),
    limits: (usize, usize),
}

impl Maze {
    fn add_byte(&mut self, loc: (usize, usize)) {
        self.map[loc.0][loc.1] = '#';
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut build_str = String::new();

        for row in &self.map {
            let mut tmp_str = String::new();
            for cha in row {
                tmp_str.push(*cha);
            }
            tmp_str.push('\n');
            build_str.push_str(&tmp_str);
        }
        write!(f, "{}", build_str)
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (initial_node, maze, _) = get_maze(file_iter, 6, 12);
            println!("{}", maze);
            assert_eq!(navigate_maze(initial_node, &maze, true).unwrap().0, 22);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (initial_node, mut maze, remaining) = get_maze(file_iter, 6, 12);
            println!("{}", maze);
            let starter_chain: Vec<(usize, usize)> = navigate_maze(initial_node.clone(), &maze, true).unwrap().1;
            assert_eq!(find_latest_drop(initial_node, &mut maze, remaining, starter_chain, true), (1,6));
        }
    }
}
