use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::Wrapping;
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_6.txt") {
        let (obs_loc, guard_loc, grid_size) = get_grid(file_iter);
        let visited: HashSet<(usize, usize)> = march_grid(&obs_loc, guard_loc, grid_size);
        println!("Part 1: {}", visited.len());
        let loop_places: HashSet<(usize, usize)> =
            find_loops(obs_loc, guard_loc, grid_size, visited);
        println!("Part 2: {}", loop_places.len());
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_grid(
    input_iter: io::Lines<io::BufReader<File>>,
) -> (HashSet<(usize, usize)>, (usize, usize), (usize, usize)) {
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut guard = (0, 0);
    let mut grid_size = (0, 0);
    for (i, line) in input_iter.enumerate() {
        grid_size.0 = i;
        for (j, character) in line.unwrap().chars().enumerate() {
            grid_size.1 = j;
            match character {
                '.' => continue,
                '#' => {
                    obstacles.insert((i, j));
                }
                '^' => {
                    guard = (i, j);
                }
                _ => {}
            }
        }
    }
    return (obstacles, guard, grid_size);
}

fn march_grid(
    obs_loc: &HashSet<(usize, usize)>,
    start_loc: (usize, usize),
    grid_size: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut current_loc = start_loc;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_dir: Direction = Direction::North;
    while current_loc.0 <= grid_size.0 && current_loc.1 <= grid_size.1 {
        visited.insert(current_loc);
        match current_dir {
            Direction::North => {
                if obs_loc.contains(&((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1)) {
                    current_dir = Direction::East;
                    continue;
                } else {
                    current_loc = ((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1);
                    continue;
                }
            }
            Direction::South => {
                if obs_loc.contains(&(current_loc.0 + 1, current_loc.1)) {
                    current_dir = Direction::West;
                    continue;
                } else {
                    current_loc = (current_loc.0 + 1, current_loc.1);
                    continue;
                }
            }
            Direction::East => {
                if obs_loc.contains(&(current_loc.0, current_loc.1 + 1)) {
                    current_dir = Direction::South;
                    continue;
                } else {
                    current_loc = (current_loc.0, current_loc.1 + 1);
                    continue;
                }
            }
            Direction::West => {
                if obs_loc.contains(&(current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0)) {
                    current_dir = Direction::North;
                    continue;
                } else {
                    current_loc = (current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0);
                    continue;
                }
            }
        }
    }
    return visited;
}

#[allow(dead_code)]
fn old_detect_loop(
    obs_loc: HashSet<(usize, usize)>,
    start_loc: (usize, usize),
    grid_size: (usize, usize),
) -> bool {
    let mut current_loc = start_loc;
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut current_dir: Direction = Direction::North;
    while current_loc.0 <= grid_size.0 && current_loc.1 <= grid_size.1 {
        visited.insert((current_loc.0, current_loc.1, current_dir));
        match current_dir {
            Direction::North => {
                if obs_loc.contains(&((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1)) {
                    current_dir = Direction::East;
                    continue;
                } else {
                    current_loc = ((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1);
                    if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            Direction::South => {
                if obs_loc.contains(&(current_loc.0 + 1, current_loc.1)) {
                    current_dir = Direction::West;
                    continue;
                } else {
                    current_loc = (current_loc.0 + 1, current_loc.1);
                    if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            Direction::East => {
                if obs_loc.contains(&(current_loc.0, current_loc.1 + 1)) {
                    current_dir = Direction::South;
                    continue;
                } else {
                    current_loc = (current_loc.0, current_loc.1 + 1);
                    if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            Direction::West => {
                if obs_loc.contains(&(current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0)) {
                    current_dir = Direction::North;
                    continue;
                } else {
                    current_loc = (current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0);
                    if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    return false;
}

fn detect_loop(
    obs_loc: HashSet<(usize, usize)>,
    start_loc: (usize, usize),
    grid_size: (usize, usize),
) -> bool {
    let mut current_loc = start_loc;
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut current_dir: Direction = Direction::North;
    'letsgo: loop {
        let mut new_loc;
        match current_dir {
            Direction::North => {
                new_loc = ((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1);
                while !obs_loc.contains(&new_loc) {
                    if new_loc.0 > grid_size.0 || new_loc.1 > grid_size.1 {
                        break 'letsgo;
                    } else {
                        current_loc = new_loc;
                    }
                    new_loc = ((Wrapping(current_loc.0) - Wrapping(1)).0, current_loc.1);
                }
                current_dir = Direction::East;
                if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                    return true;
                } else {
                    visited.insert((current_loc.0, current_loc.1, current_dir));
                    continue 'letsgo;
                }
            }
            Direction::South => {
                new_loc = (current_loc.0 + 1, current_loc.1);
                while !obs_loc.contains(&new_loc) {
                    if new_loc.0 > grid_size.0 || new_loc.1 > grid_size.1 {
                        break 'letsgo;
                    } else {
                        current_loc = new_loc;
                    }
                    new_loc = (current_loc.0 + 1, current_loc.1);
                }
                current_dir = Direction::West;
                if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                    return true;
                } else {
                    visited.insert((current_loc.0, current_loc.1, current_dir));
                    continue 'letsgo;
                }
            }
            Direction::East => {
                new_loc = (current_loc.0, current_loc.1 + 1);
                while !obs_loc.contains(&new_loc) {
                    if new_loc.0 > grid_size.0 || new_loc.1 > grid_size.1 {
                        break 'letsgo;
                    } else {
                        current_loc = new_loc;
                    }
                    new_loc = (current_loc.0, current_loc.1 + 1);
                }
                current_dir = Direction::South;
                if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                    return true;
                } else {
                    visited.insert((current_loc.0, current_loc.1, current_dir));
                    continue 'letsgo;
                }
            }
            Direction::West => {
                new_loc = (current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0);
                while !obs_loc.contains(&new_loc) {
                    if new_loc.0 > grid_size.0 || new_loc.1 > grid_size.1 {
                        break 'letsgo;
                    } else {
                        current_loc = new_loc;
                    }
                    new_loc = (current_loc.0, (Wrapping(current_loc.1) - Wrapping(1)).0);
                }
                current_dir = Direction::North;
                if visited.contains(&(current_loc.0, current_loc.1, current_dir)) {
                    return true;
                } else {
                    visited.insert((current_loc.0, current_loc.1, current_dir));
                    continue 'letsgo;
                }
            }
        }
    }
    return false;
}

fn find_loops(
    obs_loc: HashSet<(usize, usize)>,
    start_loc: (usize, usize),
    grid_size: (usize, usize),
    visited_locs: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut potential_loops: HashSet<(usize, usize)> = HashSet::new();
    for loc in visited_locs.iter() {
        let mut potential_obs = obs_loc.clone();
        potential_obs.insert(*loc);
        if detect_loop(potential_obs, start_loc, grid_size) {
            potential_loops.insert(*loc);
        }
    }
    return potential_loops;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (obs_loc, guard_loc, grid_size) = get_grid(file_iter);
            let visited: HashSet<(usize, usize)> = march_grid(&obs_loc, guard_loc, grid_size);
            assert_eq!(visited.len(), 41);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (obs_loc, guard_loc, grid_size) = get_grid(file_iter);
            let visited: HashSet<(usize, usize)> = march_grid(&obs_loc, guard_loc, grid_size);
            let loop_places: HashSet<(usize, usize)> =
                find_loops(obs_loc, guard_loc, grid_size, visited);
            assert_eq!(loop_places.len(), 6);
        }
    }
}
