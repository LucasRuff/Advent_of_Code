use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_12.txt") {
        let mut total_cost = 0;
        let mut prev_region = '~';
        let (total_cost_1, total_cost_2) = attempt_two(file_iter);
        println!("Total cost: {}", total_cost_1);
        println!("Part 2: {}", total_cost_2);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn attempt_two(ifile: io::Lines<io::BufReader<File>>) -> (usize, usize) {
    let mut fences: HashMap<(usize, usize), Fence> = HashMap::new();
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut total_input: Vec<Vec<char>> = Vec::new();
    let mut total_cost = 0;
    let mut part_2_cost = 0;
    for line in ifile {
        let text = line.unwrap();
        let mut input_row_chars: Vec<char> = text.chars().collect();
        total_input.push(input_row_chars);
    }
    let g_cols = total_input[0].len();
    let g_rows = total_input.len();
    let mut frontier: Vec<(usize, usize)> = Vec::new();

    for i in 0..g_rows {
        for j in 0..g_cols {
            if visited_locations.contains(&(i, j)) {
                continue;
            }
            frontier.push((i, j));
            let mut area = 0;
            let mut perimeter = 0;
            let region_id = total_input[i][j];
            fences.clear();
            loop {
                match frontier.pop() {
                    Some((f_i, f_j)) => {
                        if visited_locations.contains(&(f_i, f_j)) {
                            continue;
                        }
                        area += 1;
                        visited_locations.insert((f_i, f_j));
                        //look up
                        if f_i.wrapping_sub(1) < g_rows {
                            if total_input[f_i - 1][f_j] != region_id {
                                fences
                                    .entry((f_i, f_j))
                                    .and_modify(|f| f.up = true)
                                    .or_insert(Fence {
                                        up: true,
                                        down: false,
                                        left: false,
                                        right: false,
                                    });
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i - 1, f_j)) {
                                    frontier.push((f_i - 1, f_j));
                                }
                            }
                        } else {
                            fences
                                .entry((f_i, f_j))
                                .and_modify(|f| f.up = true)
                                .or_insert(Fence {
                                    up: true,
                                    down: false,
                                    left: false,
                                    right: false,
                                });
                            perimeter += 1;
                        }
                        //look down
                        if f_i + 1 < g_rows {
                            if total_input[f_i + 1][f_j] != region_id {
                                fences
                                    .entry((f_i, f_j))
                                    .and_modify(|f| f.down = true)
                                    .or_insert(Fence {
                                        up: false,
                                        down: true,
                                        left: false,
                                        right: false,
                                    });
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i + 1, f_j)) {
                                    frontier.push((f_i + 1, f_j));
                                }
                            }
                        } else {
                            fences
                                .entry((f_i, f_j))
                                .and_modify(|f| f.down = true)
                                .or_insert(Fence {
                                    up: false,
                                    down: true,
                                    left: false,
                                    right: false,
                                });
                            perimeter += 1;
                        }
                        //look right
                        if f_j + 1 < g_cols {
                            if total_input[f_i][f_j + 1] != region_id {
                                fences
                                    .entry((f_i, f_j))
                                    .and_modify(|f| f.right = true)
                                    .or_insert(Fence {
                                        up: false,
                                        down: false,
                                        left: false,
                                        right: true,
                                    });
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i, f_j + 1)) {
                                    frontier.push((f_i, f_j + 1));
                                }
                            }
                        } else {
                            fences
                                .entry((f_i, f_j))
                                .and_modify(|f| f.right = true)
                                .or_insert(Fence {
                                    up: false,
                                    down: false,
                                    left: false,
                                    right: true,
                                });
                            perimeter += 1;
                        }
                        //look left
                        if f_j.wrapping_sub(1) < g_cols {
                            if total_input[f_i][f_j - 1] != region_id {
                                fences
                                    .entry((f_i, f_j))
                                    .and_modify(|f| f.left = true)
                                    .or_insert(Fence {
                                        up: false,
                                        down: false,
                                        left: true,
                                        right: false,
                                    });
                                perimeter += 1;
                            } else {
                                if !visited_locations.contains(&(f_i, f_j - 1)) {
                                    frontier.push((f_i, f_j - 1));
                                }
                            }
                        } else {
                            fences
                                .entry((f_i, f_j))
                                .and_modify(|f| f.left = true)
                                .or_insert(Fence {
                                    up: false,
                                    down: false,
                                    left: true,
                                    right: false,
                                });
                            perimeter += 1;
                        }
                    }
                    None => break,
                }
            }

            total_cost += area * perimeter;

            part_2_cost += area * calculate_true_perimter(&fences, g_rows, g_cols);
        }
    }
    return (total_cost, part_2_cost);
}

fn calculate_true_perimter(
    fence_map: &HashMap<(usize, usize), Fence>,
    g_rows: usize,
    g_cols: usize,
) -> usize {
    let mut fused_fences = 0;
    for i in 0..g_rows {
        for j in 0..g_cols {
            match fence_map.get(&(i, j)) {
                Some(f_1) => {
                    //look up
                    match fence_map.get(&(i.wrapping_sub(1), j)) {
                        Some(f_2) => match (f_1.left, f_2.left, f_1.right, f_2.right) {
                            (false, _, false, _)
                            | (_, true, _, true)
                            | (false, _, _, true)
                            | (_, true, false, _) => {}
                            (true, false, true, true)
                            | (true, true, true, false)
                            | (true, false, false, _)
                            | (false, _, true, false) => fused_fences += 1,
                            (true, false, true, false) => fused_fences += 2,
                        },
                        None => {
                            fused_fences += match (f_1.left, f_1.right) {
                                (true, true) => 2,
                                (true, false) | (false, true) => 1,
                                (false, false) => 0,
                            };
                        }
                    }
                    //look left
                    match fence_map.get(&(i, j.wrapping_sub(1))) {
                        Some(f_2) => match (f_1.up, f_2.up, f_1.down, f_2.down) {
                            (false, _, false, _)
                            | (_, true, _, true)
                            | (false, _, _, true)
                            | (_, true, false, _) => {}
                            (true, false, true, true)
                            | (true, true, true, false)
                            | (true, false, false, _)
                            | (false, _, true, false) => fused_fences += 1,
                            (true, false, true, false) => fused_fences += 2,
                        },
                        None => {
                            fused_fences += match (f_1.up, f_1.down) {
                                (true, true) => 2,
                                (true, false) | (false, true) => 1,
                                (false, false) => 0,
                            };
                        }
                    }
                }
                None => continue,
            }
        }
    }
    return fused_fences;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Clone)]
struct Fence {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_two_test() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            assert_eq!(attempt_two(file_iter), (1930, 1206));
        }
    }
    #[test]
    fn attempt_two_part_2() {
        if let Ok(file_iter) = read_lines("easy_test.txt") {
            assert_eq!(attempt_two(file_iter).1, 368);
        }
    }
}
