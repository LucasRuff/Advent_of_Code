use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_8.txt") {
        let (all_keys, all_networks, grid_size) = create_grid(file_iter);
        let mut some_antinodes: HashSet<(isize, isize)> = HashSet::new();
        let mut more_antinodes: HashSet<(isize, isize)> = HashSet::new();
        for key in all_keys {
            let all_antennas = all_networks.get(&key).unwrap();
            for i in 0..all_antennas.len() - 1 {
                for j in i + 1..all_antennas.len() {
                    match get_antinodes(all_antennas[i], all_antennas[j], grid_size) {
                        (None, Some(loc)) | (Some(loc), None) => {
                            some_antinodes.insert(loc);
                        }
                        (Some(loc1), Some(loc2)) => {
                            some_antinodes.insert(loc1);
                            some_antinodes.insert(loc2);
                        }
                        (None, None) => {}
                    }
                    for antinode in get_more_antinodes(all_antennas[i], all_antennas[j], grid_size)
                    {
                        more_antinodes.insert(antinode);
                    }
                }
            }
        }
        println!("Part 1 antinodes: {:?}", some_antinodes.len());
        println!("Part 2 antinodes: {:?}", more_antinodes.len());
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

fn create_grid(
    lines: io::Lines<io::BufReader<File>>,
) -> (Vec<char>, HashMap<char, Vec<Antenna>>, (isize, isize)) {
    let mut grid_size: (isize, isize) = (0, 0);
    let mut all_networks: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut all_keys: Vec<char> = Vec::new();
    for (i, line) in lines.enumerate() {
        grid_size.0 = 0;
        for (j, cha) in line.unwrap().chars().enumerate() {
            match cha {
                '.' => {}
                _ => {
                    all_networks
                        .entry(cha)
                        .and_modify(|net| {
                            net.push(Antenna {
                                x: j.try_into().unwrap(),
                                y: i.try_into().unwrap(),
                            })
                        })
                        .or_insert(vec![Antenna {
                            x: j.try_into().unwrap(),
                            y: i.try_into().unwrap(),
                        }]);
                    if !all_keys.contains(&cha) {
                        all_keys.push(cha)
                    };
                }
            };
            grid_size.0 += 1;
        }
        grid_size.1 += 1;
    }
    return (all_keys, all_networks, grid_size);
}

fn get_more_antinodes(
    antenna_1: Antenna,
    antenna_2: Antenna,
    map_size: (isize, isize),
) -> Vec<(isize, isize)> {
    let dist_x: isize = antenna_1.x - antenna_2.x;
    let dist_y: isize = antenna_1.y - antenna_2.y;
    let mut antinodes: Vec<(isize, isize)> = Vec::new();
    let mut mult = 0;
    'forwards: loop {
        let try_x = mult * dist_x + antenna_1.x;
        let try_y = mult * dist_y + antenna_1.y;
        match (
            (try_x >= map_size.0 || try_x < 0),
            (try_y >= map_size.1 || try_y < 0),
        ) {
            (true, _) | (_, true) => break 'forwards,
            (false, false) => {
                antinodes.push((try_x, try_y));
            }
        }
        mult += 1;
    }
    mult = 0;
    'backwards: loop {
        let try_x = mult * dist_x + antenna_1.x;
        let try_y = mult * dist_y + antenna_1.y;
        match (
            (try_x >= map_size.0 || try_x < 0),
            (try_y >= map_size.1 || try_y < 0),
        ) {
            (true, _) | (_, true) => break 'backwards,
            (false, false) => {
                antinodes.push((try_x, try_y));
            }
        }
        mult -= 1;
    }

    return antinodes;
}

fn get_antinodes(
    antenna_1: Antenna,
    antenna_2: Antenna,
    map_size: (isize, isize),
) -> (Option<(isize, isize)>, Option<(isize, isize)>) {
    let try_x: isize = 2 * antenna_1.x - antenna_2.x;
    let try_y: isize = 2 * antenna_1.y - antenna_2.y;
    let antinode_1: Option<(isize, isize)> = match (
        (try_x >= map_size.0 || try_x < 0),
        (try_y >= map_size.1 || try_y < 0),
    ) {
        (true, _) | (_, true) => None,
        (false, false) => Some((try_x, try_y)),
    };
    let try_x: isize = 2 * antenna_2.x - antenna_1.x;
    let try_y: isize = 2 * antenna_2.y - antenna_1.y;
    let antinode_2: Option<(isize, isize)> = match (
        (try_x >= map_size.0 || try_x < 0),
        (try_y >= map_size.1 || try_y < 0),
    ) {
        (true, _) | (_, true) => None,
        (false, false) => Some((try_x, try_y)),
    };

    return (antinode_1, antinode_2);
}

#[derive(Clone, Copy, Debug)]
struct Antenna {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let mut total = 0;
            let (all_keys, all_networks, grid_size) = create_grid(file_iter);
            let mut all_antinodes: HashSet<(isize, isize)> = HashSet::new();
            for key in all_keys {
                let all_antennas = all_networks.get(&key).unwrap();
                for i in 0..all_antennas.len() - 1 {
                    for j in i + 1..all_antennas.len() {
                        match get_antinodes(all_antennas[i], all_antennas[j], grid_size) {
                            (None, Some(loc)) | (Some(loc), None) => {
                                all_antinodes.insert(loc);
                            }
                            (Some(loc1), Some(loc2)) => {
                                all_antinodes.insert(loc1);
                                all_antinodes.insert(loc2);
                            }
                            (None, None) => continue,
                        };
                    }
                }
            }
            assert_eq!(all_antinodes.len(), 14);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_2.txt") {
            let mut total = 0;
            let (all_keys, all_networks, grid_size) = create_grid(file_iter);
            let mut all_antinodes: HashSet<(isize, isize)> = HashSet::new();
            for key in all_keys {
                if key == 'T' {
                    let all_antennas = all_networks.get(&key).unwrap();
                    for i in 0..all_antennas.len() - 1 {
                        for j in i + 1..all_antennas.len() {
                            for antinode in
                                get_more_antinodes(all_antennas[i], all_antennas[j], grid_size)
                            {
                                all_antinodes.insert(antinode);
                            }
                        }
                    }
                }
            }
            let known_antinodes: HashSet<(isize, isize)> = HashSet::from([
                (0, 0),
                (5, 0),
                (3, 1),
                (1, 2),
                (6, 2),
                (9, 3),
                (2, 4),
                (3, 6),
                (4, 8),
            ]);
            assert_eq!(all_antinodes, known_antinodes);
        }
    }

    #[test]
    fn test_get_antinode() {
        let map_size: (isize, isize) = (6, 6);
        let antenna_1 = Antenna { x: 3, y: 2 };
        let antenna_2 = Antenna { x: 5, y: 3 };
        assert_eq!(
            get_antinodes(antenna_1, antenna_2, map_size),
            (Some((1, 1)), None)
        );
    }
}
