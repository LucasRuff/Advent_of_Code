use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_10.txt") {
        let (trailheads, map) = process_map(file_iter);
        let mut total = 0;
        let mut rating = 0;
        for head in trailheads {
            total += get_peaks(head.clone(), &map).len();
            rating += get_rating(head, &map);
        }
        println!("total 1: {}", total);
        println!("total 2: {}", rating);
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

fn get_rating(trailhead: Location, map: &Vec<Vec<u32>>) -> u32 {
    if trailhead.z == 9 {
        let mut t: Vec<Location> = Vec::new();
        t.push(trailhead);
        return 1;
    } else {
        let mut total = 0;
        if trailhead.x < map[0].len() - 1 {
            if trailhead.z + 1 == map[trailhead.y][trailhead.x + 1] {
                total += get_rating(
                    Location {
                        x: trailhead.x + 1,
                        y: trailhead.y,
                        z: trailhead.z + 1,
                    },
                    map,
                );
            }
        }
        if trailhead.x > 0 {
            if trailhead.z + 1 == map[trailhead.y][trailhead.x - 1] {
                total += get_rating(
                    Location {
                        x: trailhead.x - 1,
                        y: trailhead.y,
                        z: trailhead.z + 1,
                    },
                    map,
                );
            }
        }
        if trailhead.y < map.len() - 1 {
            if trailhead.z + 1 == map[trailhead.y + 1][trailhead.x] {
                total += get_rating(
                    Location {
                        x: trailhead.x,
                        y: trailhead.y + 1,
                        z: trailhead.z + 1,
                    },
                    map,
                );
            }
        }
        if trailhead.y > 0 {
            if trailhead.z + 1 == map[trailhead.y - 1][trailhead.x] {
                total += get_rating(
                    Location {
                        x: trailhead.x,
                        y: trailhead.y - 1,
                        z: trailhead.z + 1,
                    },
                    map,
                );
            }
        }

        return total;
    }
}

fn get_peaks(trailhead: Location, map: &Vec<Vec<u32>>) -> HashSet<Location> {
    if trailhead.z == 9 {
        let mut t: HashSet<Location> = HashSet::new();
        t.insert(trailhead);
        return t;
    } else {
        let mut next_peaks: HashSet<Location> = HashSet::new();
        if trailhead.x < map[0].len() - 1 {
            if trailhead.z + 1 == map[trailhead.y][trailhead.x + 1] {
                for peak in get_peaks(
                    Location {
                        x: trailhead.x + 1,
                        y: trailhead.y,
                        z: trailhead.z + 1,
                    },
                    map,
                ) {
                    next_peaks.insert(peak);
                }
            }
        }
        if trailhead.x > 0 {
            if trailhead.z + 1 == map[trailhead.y][trailhead.x - 1] {
                for peak in get_peaks(
                    Location {
                        x: trailhead.x - 1,
                        y: trailhead.y,
                        z: trailhead.z + 1,
                    },
                    map,
                ) {
                    next_peaks.insert(peak);
                }
            }
        }
        if trailhead.y < map.len() - 1 {
            if trailhead.z + 1 == map[trailhead.y + 1][trailhead.x] {
                for peak in get_peaks(
                    Location {
                        x: trailhead.x,
                        y: trailhead.y + 1,
                        z: trailhead.z + 1,
                    },
                    map,
                ) {
                    next_peaks.insert(peak);
                }
            }
        }
        if trailhead.y > 0 {
            if trailhead.z + 1 == map[trailhead.y - 1][trailhead.x] {
                for peak in get_peaks(
                    Location {
                        x: trailhead.x,
                        y: trailhead.y - 1,
                        z: trailhead.z + 1,
                    },
                    map,
                ) {
                    next_peaks.insert(peak);
                }
            }
        }

        return next_peaks;
    }
}

fn process_map(ifile: io::Lines<io::BufReader<File>>) -> (Vec<Location>, Vec<Vec<u32>>) {
    let mut map = Vec::new();
    let mut trailheads = Vec::new();
    for (i, line) in ifile.enumerate() {
        let text = line.unwrap();
        let mut map_line = Vec::new();
        for (j, cha) in text.chars().enumerate() {
            map_line.push(cha.to_digit(10).unwrap());
            if cha == '0' {
                trailheads.push(Location { x: j, y: i, z: 0 });
            }
        }
        map.push(map_line);
    }
    return (trailheads, map);
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Location {
    x: usize,
    y: usize,
    z: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (trailheads, map) = process_map(file_iter);
            let mut total = 0;
            for head in trailheads {
                total += get_peaks(head, &map).len();
            }
            assert_eq!(total, 36);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (trailheads, map) = process_map(file_iter);
            let mut total = 0;
            for head in trailheads {
                total += get_rating(head, &map);
            }
            assert_eq!(total, 81);
        }
    }

    #[test]
    fn test_get_peaks() {
        let trailhead_1 = Location { x: 0, y: 0, z: 0 };
        let map = vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ];
        let mut goal_set = HashSet::new();
        goal_set.insert(Location { x: 0, y: 3, z: 9 });
        assert_eq!(get_peaks(trailhead_1, &map), goal_set);
    }

    #[test]
    fn test_get_map() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (_, map) = process_map(file_iter);
            let target_map = vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ];
            assert_eq!(map, target_map);
        }
    }

    #[test]
    fn test_get_trailheads() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (i_heads, _) = process_map(file_iter);

            let trailheads = vec![
                Location { x: 2, y: 0, z: 0 },
                Location { x: 4, y: 0, z: 0 },
                Location { x: 4, y: 2, z: 0 },
                Location { x: 6, y: 4, z: 0 },
                Location { x: 2, y: 5, z: 0 },
                Location { x: 5, y: 5, z: 0 },
                Location { x: 0, y: 6, z: 0 },
                Location { x: 6, y: 6, z: 0 },
                Location { x: 1, y: 7, z: 0 },
            ];

            assert_eq!(i_heads, trailheads);
        }
    }
}
