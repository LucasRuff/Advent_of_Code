use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let now = Instant::now();
    let mut total: usize = 0;
    if let Ok(file_iter) = read_lines("input_17.txt") {
        let mut city_map = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                city_map.push(text.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>());
            }
        }

        let map_width = city_map[0].len();
        let map_height = city_map.len();
        let mut frontier = Vec::new();
        frontier.push(Node{
            location: (0,4),
            cost: city_map[0][1] + city_map[0][2] + city_map[0][3] + city_map[0][4],
            heuristic: city_map[0][1] + city_map[0][2] + city_map[0][3] + city_map[0][4] + manhattan_distance((0,4), map_width, map_height),
            straights: 4,
            direction: Direction::East,
        });
        frontier.push(Node{
            location: (4,0),
            cost: city_map[1][0] + city_map[2][0] + city_map[3][0] + city_map[4][0],
            heuristic: city_map[1][0] + city_map[2][0] + city_map[3][0] + city_map[4][0] + manhattan_distance((1,0), map_width, map_height),
            straights: 4,
            direction: Direction::South,
        });
        frontier.sort();
        frontier.reverse();
        let mut visited = HashSet::new();
        while frontier.len() > 0 {
            let Some(current_node) = frontier.pop() else {panic!("Error: node could not be read");};
            //println!("Processing node ({}, {}) with heuristic {}, straights {}, direction {:?}", current_node.location.0, current_node.location.1, current_node.heuristic, current_node.straights, current_node.direction);
            if current_node.location == (map_height - 1, map_width - 1) {
                total = current_node.cost;
                break;
            }
            // if visited.get(&(current_node.location, current_node.direction, current_node.straights)).is_some() {
            //     //println!("Repeating!");
            //     continue 'bigloop;
            // }

            match current_node.direction {
                Direction::East => {
                    if current_node.location.1 < map_width - 1 && current_node.straights < 10 {
                        let new_location = (current_node.location.0, current_node.location.1 + 1);
                        let new_cost = current_node.cost + city_map[new_location.0][new_location.1];
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: (current_node.straights + 1), direction: (Direction::East) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.0 > 3 {
                        let new_location = (current_node.location.0 - 4, current_node.location.1);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0 + i][new_location.1];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: Direction::North };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.0 < map_height - 4 {
                        let new_location = (current_node.location.0 + 4, current_node.location.1);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0 - i][new_location.1];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: Direction::South };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                },
                Direction::North => {
                    if current_node.location.0 > 0 && current_node.straights < 10 {
                        let new_location = (current_node.location.0 - 1, current_node.location.1);
                        let new_cost = current_node.cost + city_map[new_location.0][new_location.1];
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: current_node.straights + 1, direction: Direction::North };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.1 < map_width - 4 {
                        let new_location = (current_node.location.0, current_node.location.1 + 4);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0][new_location.1 - i];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: (Direction::East) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.1 > 3 {
                        let new_location = (current_node.location.0, current_node.location.1 - 4);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0][new_location.1 + i];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: (Direction::West) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                },
                Direction::South => {
                    if current_node.location.0 < map_height - 1 && current_node.straights < 10 {
                        let new_location = (current_node.location.0 + 1, current_node.location.1);
                        let new_cost = current_node.cost + city_map[new_location.0][new_location.1];
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: current_node.straights + 1, direction: Direction::South };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.1 < map_width - 4 {
                        let new_location = (current_node.location.0, current_node.location.1 + 4);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0][new_location.1 - i];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: (Direction::East) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.1 > 3 {
                        let new_location = (current_node.location.0, current_node.location.1 - 4);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0][new_location.1 + i];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: (Direction::West) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                },
                Direction::West => {
                    if current_node.location.1 > 0 && current_node.straights < 10 {
                        let new_location = (current_node.location.0, current_node.location.1 - 1);
                        let new_cost = current_node.cost + city_map[new_location.0][new_location.1];
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: current_node.straights + 1, direction: (Direction::West) };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.0 > 3 {
                        let new_location = (current_node.location.0 - 4, current_node.location.1);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0 + i][new_location.1];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: Direction::North };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                    if current_node.location.0 < map_height - 4 {
                        let new_location = (current_node.location.0 + 4, current_node.location.1);
                        let mut new_cost = current_node.cost;
                        for i in 0..4 {
                            new_cost += city_map[new_location.0 - i][new_location.1];
                        }
                        let new_node = Node { location: new_location, cost: new_cost, heuristic: new_cost + map_width + map_height - new_location.0 - new_location.1 - 2, straights: 4, direction: Direction::South };
                        if visited.get(&(new_node.location, new_node.direction, new_node.straights)).is_none() {
                            visited.insert((new_node.location, new_node.direction, new_node.straights));
                            frontier.push(new_node);
                        }
                    }
                },
            }
            //visited.insert((current_node.location, current_node.direction, current_node.straights));

            frontier.sort_by(|a,b| b.cmp(a));
        }
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn manhattan_distance(location: (usize, usize), map_width: usize, map_height: usize) -> usize {
    map_width + map_height - location.0 - location.1 - 2
}

#[derive(PartialEq, Eq, Hash)]
struct Node {
    location: (usize, usize),
    cost: usize,
    heuristic: usize,
    straights: usize,
    direction: Direction,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.heuristic.partial_cmp(&other.heuristic);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}