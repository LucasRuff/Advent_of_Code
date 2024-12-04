use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let now = Instant::now();
    let mut total: u64 = 0;
    if let Ok(file_iter) = read_lines("input_16.txt") {
        let mut mechanism_rows: Vec<Vec<char>> = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                mechanism_rows.push(text.chars().collect());
            }
        }
        //println!("energized grid: {:?}", energized_grid);
        for i in 0..mechanism_rows[0].len() {
            let tmp = illuminate_field(&mechanism_rows, Beam{location: (0,i), dir: Direction::South});
            if tmp > total {
                total = tmp;
            }
            let tmp = illuminate_field(&mechanism_rows, Beam{location: (mechanism_rows.len()-1,i), dir: Direction::North});
            if tmp > total {
                total = tmp;
            }
        }
        for j in 0..mechanism_rows.len() {
            let tmp = illuminate_field(&mechanism_rows, Beam{location: (j,0), dir: Direction::East});
            if tmp > total {
                total = tmp;
            }
            let tmp = illuminate_field(&mechanism_rows, Beam{location: (j,mechanism_rows[0].len()-1), dir: Direction::West});
            if tmp > total {
                total = tmp;
            }
        }


        
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn illuminate_field(mechanism: &Vec<Vec<char>>, start_beam: Beam) -> u64 {
    let mut total: u64 = 0;
    let grid_width = mechanism[0].len();
    let grid_height = mechanism.len();
    let mut energized_grid = mechanism.clone();
    let mut beam_frontier = Vec::new();
    beam_frontier.push(start_beam);
    let mut visited_cells: HashSet<Beam> = HashSet::new();
    while beam_frontier.len() > 0 {
        let Some(next_beam) = beam_frontier.pop() else { panic!("could not read beam"); };
        match visited_cells.get(&next_beam) {
            Some(_) => { continue; },
            None => {
                energized_grid[next_beam.location.0][next_beam.location.1] = '1';
                //pretty_print(&energized_grid);
                match mechanism[next_beam.location.0][next_beam.location.1] {
                    '.' => {
                        match next_beam.dir {
                            Direction::North => {
                                if next_beam.location.0 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 - 1, next_beam.location.1), dir: Direction::North });
                                }
                            },
                            Direction::South => {
                                if next_beam.location.0 < grid_height - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 + 1, next_beam.location.1), dir: Direction::South });
                                }
                            },
                            Direction::East => {
                                if next_beam.location.1 < grid_width - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 + 1), dir: Direction::East });
                                }
                            },
                            Direction::West => {
                                if next_beam.location.1 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 - 1), dir: Direction::West});
                                }
                            }
                        }
                    },
                    '|' => {
                        if next_beam.dir == Direction::East || next_beam.dir == Direction::West {
                            if next_beam.location.0 > 0 {
                                beam_frontier.push(Beam{location: (next_beam.location.0 - 1, next_beam.location.1), dir: Direction::North });
                            }
                            if next_beam.location.0 < grid_height - 1 {
                                beam_frontier.push(Beam{location: (next_beam.location.0 + 1, next_beam.location.1), dir: Direction::South });
                            }
                        } else {
                            match next_beam.dir {
                                Direction::North => {
                                    if next_beam.location.0 > 0 {
                                        beam_frontier.push(Beam{location: (next_beam.location.0 - 1, next_beam.location.1), dir: Direction::North });
                                    }
                                },
                                Direction::South => {
                                    if next_beam.location.0 < grid_height - 1 {
                                        beam_frontier.push(Beam{location: (next_beam.location.0 + 1, next_beam.location.1), dir: Direction::South});
                                    }
                                },
                                _ => {},
                            }
                        }
                    },
                    '-' => {
                        if next_beam.dir == Direction::North || next_beam.dir == Direction::South {
                            if next_beam.location.1 < grid_width - 1 {
                                beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 + 1), dir: Direction::East });
                            }
                            if next_beam.location.1 > 0 {
                                beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 - 1), dir: Direction::West});
                            }
                        } else {
                            match next_beam.dir {
                                Direction::East => {
                                    if next_beam.location.1 < grid_width - 1 {
                                        beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 + 1), dir: Direction::East });
                                    }
                                },
                                Direction::West => {
                                    if next_beam.location.1 > 0 {
                                        beam_frontier.push(Beam{location: (next_beam.location.0,  next_beam.location.1 - 1), dir: Direction::West });
                                    }
                                },
                                _ => {},
                            }
                        }
                    },
                    '\\' => {
                        match next_beam.dir {
                            Direction::North => {
                                if next_beam.location.1 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 - 1), dir: Direction::West });
                                }
                            },
                            Direction::South => {
                                if next_beam.location.1 < grid_width - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 + 1), dir: Direction::East });
                                }
                            },
                            Direction::East => {
                                if next_beam.location.0 < grid_height - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 + 1, next_beam.location.1), dir: Direction::South });
                                }
                            },
                            Direction::West => {
                                if next_beam.location.0 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 - 1, next_beam.location.1), dir: Direction::North});
                                }
                            }
                        }
                    },
                    '/' => {
                        match next_beam.dir {
                            Direction::North => {
                                if next_beam.location.1 < grid_width - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 + 1), dir: Direction::East });
                                }
                            },
                            Direction::South => {
                                if next_beam.location.1 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0, next_beam.location.1 - 1), dir: Direction::West });
                                }
                            },
                            Direction::East => {
                                if next_beam.location.0 > 0 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 - 1, next_beam.location.1), dir: Direction::North});
                                }
                            },
                            Direction::West => {
                                if next_beam.location.0 < grid_height - 1 {
                                    beam_frontier.push(Beam{location: (next_beam.location.0 + 1, next_beam.location.1), dir: Direction::South });
                                }
                            }
                        }
                    },
                    _ => {panic!("could not read mechanism")}
                }
                visited_cells.insert(next_beam);
            }
        }
    }
    for row in energized_grid {
        for cha in row {
            total += cha.to_string().parse::<u64>().unwrap_or(0);
        }
    }
    total
}

// fn pretty_print(input_vec: &Vec<Vec<char>>) {
//     for row in input_vec {
//         let tmp_str: String = row.iter().collect::<String>();
//         println!("{}", tmp_str);
//     }
// }

#[derive(Hash, Eq, PartialEq)]
struct Beam {
    location: (usize, usize),
    dir: Direction,
}
#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}