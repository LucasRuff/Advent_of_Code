use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_16.txt") {
        let (initial_node, maze) = get_maze(file_iter);
        println!("totals: {:?}", navigate_maze(initial_node, maze, false));
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_maze(i_file: io::Lines<io::BufReader<File>>) -> (Node, Maze) {
    let mut maze: Vec<Vec<char>> = Vec::new();
    let (mut goal_x, mut goal_y): (usize, usize) = (0, 0);
    let mut initial_x: usize = 0;
    let mut initial_y: usize = 0;
    for (i, line) in i_file.enumerate() {
        let text = line.unwrap();
        let mut row_buffer: Vec<char> = Vec::new();
        for (j, cha) in text.chars().enumerate() {
            if cha == 'E' {
                goal_x = j;
                goal_y = i;
                row_buffer.push('.');
            } else if cha == 'S' {
                initial_x = j;
                initial_y = i;
                row_buffer.push('.');
            } else {
                row_buffer.push(cha);
            }
        }
        maze.push(row_buffer);
    }
    return (
        Node::origin((initial_y, initial_x)),
        Maze {
            map: maze,
            goal: (goal_y, goal_x),
        },
    );
}

fn navigate_maze(start_node: Node, mut maze: Maze, debug: bool) -> (usize, usize) {
    let mut visited_nodes: HashMap<
        /*location*/ (usize, usize, Direction),
        (
            /*cost to go*/ usize,
            /*possible parents*/ Vec<(usize, usize, Direction)>,
        ),
    > = HashMap::new();
    let mut frontier: Vec<Node> = vec![start_node];
    let mut best_cost: usize = std::usize::MAX;
    let mut seating: usize = 1;
    loop {
        if let Some(current_node) = frontier.pop() {
            if debug {
                println!("visiting node {}, {}", current_node.p_y, current_node.p_x);
            }
            if (current_node.p_y, current_node.p_x) == maze.goal {
                best_cost = if current_node.cost_to_go <= best_cost {
                    mark_seating(&current_node, &visited_nodes, &mut maze, debug);
                    current_node.cost_to_go
                } else {
                    best_cost
                };

                if debug {
                    println!("{}", maze);
                }
                continue;
            }

            let children = current_node.generate_children(&maze);
            for child in children {
                match visited_nodes.get(&(child.p_y, child.p_x, child.dir)) {
                    Some(c) => {
                        if c.0 >= child.cost_to_go {
                            visited_nodes
                                .entry((child.p_y, child.p_x, child.dir))
                                .and_modify(|d| d.1.push(child.parent));
                            frontier.push(child);
                        }
                    }
                    None => {
                        visited_nodes.insert(
                            (child.p_y, child.p_x, child.dir),
                            (child.cost_to_go, vec![child.parent]),
                        );
                        if child.cost_to_go <= best_cost {
                            frontier.push(child);
                        }
                    }
                }
            }
            frontier.sort();
            frontier.reverse();
            if debug {
                println!("Current frontier:\n{:?}", frontier);
            }
        } else {
            break;
        }
    }
    for (i, row) in maze.map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if maze.map[i][j] == 'O' {
                seating += 1;
            }
        }
    }
    return (best_cost, seating);
}

fn mark_seating(
    end_node: &Node,
    node_map: &HashMap<(usize, usize, Direction), (usize, Vec<(usize, usize, Direction)>)>,
    maze: &mut Maze,
    debug: bool,
) {
    let mut parent_front: Vec<(usize, usize, Direction)> = vec![end_node.parent];
    let mut visited_parents: HashSet<(usize, usize, Direction)> = HashSet::new();
    loop {
        match parent_front.pop() {
            Some(current_parent) => {
                maze.map[current_parent.0][current_parent.1] = 'O';
                match &node_map.get(&current_parent) {
                    Some((_, parents)) => {
                        for parent in parents {
                            if *parent != (0, 0, Direction::Right)
                                && visited_parents.get(&parent).is_none()
                            {
                                parent_front.push(*parent);
                                visited_parents.insert(*parent);
                            }
                        }
                        if debug {
                            println!(
                                "Node {:?} is on best path, parent of {:?}",
                                parents, current_parent
                            );
                            //println!("Current parent frontier: {:?}", parent_front);
                        }
                    }
                    None => continue,
                }
            }
            None => return,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug, Eq)]
struct Node {
    p_x: usize,
    p_y: usize,
    dir: Direction,
    cost_to_go: usize,
    heuristic: usize,
    parent: (usize, usize, Direction),
}

impl Node {
    fn new(parent: &Node, direction: Direction, goal: (usize, usize)) -> Node {
        let px = parent.p_x + if direction == Direction::Right { 1 } else { 0 }
            - if direction == Direction::Left { 1 } else { 0 };
        let py = parent.p_y + if direction == Direction::Down { 1 } else { 0 }
            - if direction == Direction::Up { 1 } else { 0 };
        let cost = parent.cost_to_go + if direction == parent.dir { 1 } else { 1001 };
        let heur = goal.1 - px + py - goal.0 + cost;

        Node {
            p_x: px,
            p_y: py,
            dir: direction,
            cost_to_go: cost,
            heuristic: heur,
            parent: (parent.p_y, parent.p_x, parent.dir),
        }
    }

    fn origin(point: (usize, usize)) -> Node {
        Node {
            p_x: point.1,
            p_y: point.0,
            dir: Direction::Right,
            cost_to_go: 0,
            heuristic: 0,
            parent: (0, 0, Direction::Right),
        }
    }

    fn generate_children(&self, maze: &Maze) -> Vec<Node> {
        let mut child_vec: Vec<Node> = Vec::new();
        if maze.map[self.p_y][self.p_x + 1] != '#' && self.dir != Direction::Left {
            child_vec.push(Node::new(&self, Direction::Right, maze.goal));
        }
        if maze.map[self.p_y][self.p_x - 1] != '#' && self.dir != Direction::Right {
            child_vec.push(Node::new(&self, Direction::Left, maze.goal));
        }
        if maze.map[self.p_y + 1][self.p_x] != '#' && self.dir != Direction::Up {
            child_vec.push(Node::new(&self, Direction::Down, maze.goal));
        }
        if maze.map[self.p_y - 1][self.p_x] != '#' && self.dir != Direction::Down {
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
            let (initial_node, maze) = get_maze(file_iter);
            println!("{}", maze);
            assert_eq!(maze.goal, (1, 13));
            assert_eq!(initial_node.heuristic, 0);
            assert_eq!((initial_node.p_x, initial_node.p_y), (1, 13));
            assert_eq!(navigate_maze(initial_node, maze, true).0, 7036);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (initial_node, maze) = get_maze(file_iter);
            assert_eq!(navigate_maze(initial_node, maze, true).1, 45);
        }
    }
}
