use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let mut total: u64 = 0;
    let mut history: HashMap<(Vec<Vec<char>>, Direction), Vec<Vec<char>>> = HashMap::new();
    let mut vec_history: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    let mut cache_hits = (0,0,0,0);
    if let Ok(file_iter) = read_lines("input_14.txt") {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                rows.push(text.chars().collect());
            }
        }
        let mut after_roll = rows.clone();
        let mut run_in_length = 0;
        let loop_start_field;
        'detect_cycle: loop {
            (after_roll, cache_hits) = roll_field(&mut after_roll, Direction::North, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, cache_hits) = roll_field(&mut after_roll, Direction::West, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, cache_hits) = roll_field(&mut after_roll, Direction::South, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, cache_hits) = roll_field(&mut after_roll, Direction::East, &mut history, &mut vec_history, &mut cache_hits);
            run_in_length += 1;
            if cache_hits.2 > 0 {
                loop_start_field = after_roll.clone();
                break 'detect_cycle;
            }
        }
        let mut loop_length = 1;
        (after_roll, _) = roll_field(&mut after_roll, Direction::North, &mut history, &mut vec_history, &mut cache_hits);
        (after_roll, _) = roll_field(&mut after_roll, Direction::West, &mut history, &mut vec_history, &mut cache_hits);
        (after_roll, _) = roll_field(&mut after_roll, Direction::South, &mut history, &mut vec_history, &mut cache_hits);
        (after_roll, _) = roll_field(&mut after_roll, Direction::East, &mut history, &mut vec_history, &mut cache_hits);
        while after_roll != loop_start_field {
            (after_roll, _) = roll_field(&mut after_roll, Direction::North, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::West, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::South, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::East, &mut history, &mut vec_history, &mut cache_hits);
            loop_length += 1;
        }
        let mut target_iteration = (1000000000 - run_in_length) % loop_length;
        while target_iteration > 0 {
            (after_roll, _) = roll_field(&mut after_roll, Direction::North, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::West, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::South, &mut history, &mut vec_history, &mut cache_hits);
            (after_roll, _) = roll_field(&mut after_roll, Direction::East, &mut history, &mut vec_history, &mut cache_hits);
            target_iteration -= 1;
        }

        total = score_field(&after_roll);
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn score_field(field: &Vec<Vec<char>>) -> u64 {
    let mut score = 0;
    for col in transpose(field).iter() {
        score += score_col(col);
    }
    score
}

fn score_col(col: &Vec<char>) -> u64 {
    let mut score = 0;
    for i in 0..col.len() {
        if col[i] == 'O' {
            score += col.len() - i;
        }
    }
    score as u64
}

fn roll_field(field: &mut Vec<Vec<char>>, dir: Direction, hist: &mut HashMap<(Vec<Vec<char>>, Direction), Vec<Vec<char>>>, vec_hist: &mut HashMap<Vec<char>, Vec<char>>, cache_stats: &mut (usize, usize, usize, usize)) -> (Vec<Vec<char>>, (usize, usize, usize, usize)) {
    let original_field = field.clone();
    let (mut small_cache_hits, mut small_cache_misses, mut big_cache_hits, mut big_cache_misses) = cache_stats;
    match hist.get(&(original_field.clone(), dir.clone())) {
        Some(res) => { 
            big_cache_hits += 1;
            (res.to_vec(), (small_cache_hits, small_cache_misses, big_cache_hits, big_cache_misses)) 
        },
        None => {
            big_cache_misses += 1;
            match dir {
                Direction::North => {
                    let mut cols = transpose(field);
                    for i in 0..cols.len() {
                        let current_vec = cols[i].clone();
                        cols[i] = match vec_hist.get(&current_vec.clone()) {
                            Some(res) => {
                                small_cache_hits += 1;
                                let m = res.clone();
                                m.to_vec()
                            },
                            None => {
                                small_cache_misses += 1;
                                let m = roll_vec(&mut current_vec.clone());
                                vec_hist.insert(current_vec, m.clone());
                                m.to_vec()
                            },
                        };
                    }
                    *field = transpose(&cols)
                },
                Direction::South => {
                    let mut cols = transpose(field);
                    for i in 0..cols.len() {
                        let mut current_vec = cols[i].clone();
                        current_vec.reverse();
                        cols[i] = match vec_hist.get(&current_vec.clone()) {
                            Some(res) => {
                                small_cache_hits += 1;
                                let mut m = res.clone();
                                m.reverse();
                                m.to_vec()
                            },
                            None => {
                                small_cache_misses += 1;
                                let mut m = roll_vec(&mut current_vec.clone());
                                vec_hist.insert(current_vec, m.clone());
                                m.reverse();
                                m.to_vec()
                            },
                        };
                    }
                    *field = transpose(&cols)
                },
                Direction::East => {
                    for i in 0..field.len() {
                        let mut current_vec = field[i].clone();
                        current_vec.reverse();
                        field[i] = match vec_hist.get(&current_vec.clone()) {
                            Some(res) => {
                                small_cache_hits += 1;
                                let mut m = res.clone();
                                m.reverse();
                                m.to_vec()
                            },
                            None => {
                                small_cache_misses += 1;
                                let mut m = roll_vec(&mut current_vec.clone());
                                vec_hist.insert(current_vec, m.clone());
                                m.reverse();
                                m.to_vec()
                            }
                        }
                    }
                },
                Direction::West => {
                    for i in 0..field.len() {
                        let current_vec = field[i].clone();
                        field[i] = match vec_hist.get(&current_vec.clone()) {
                            Some(res) => {
                                small_cache_hits += 1;
                                res.to_vec()
                            },
                            None => {
                                small_cache_misses += 1;
                                let m = roll_vec(&mut current_vec.clone());
                                vec_hist.insert(current_vec, m.clone());
                                m.to_vec()
                            },
                        };
                    }
                },
            }
            hist.insert((original_field, dir), field.to_vec());
            (field.to_vec(), (small_cache_hits, small_cache_misses, big_cache_hits, big_cache_misses))
        }
    }
}
    

fn roll_vec(vec: &mut Vec<char>) -> Vec<char> {
    'roll_loop: loop {
        let mut num_changes = 0;
        for i in 1..vec.len() {
            if vec[i] == 'O' && vec[i-1] == '.' {
                vec[i-1] = 'O';
                vec[i] = '.';
                num_changes += 1;
            }
        }
        if num_changes == 0 {
            break 'roll_loop;
        }
    }
    return vec.to_vec();
}

fn transpose(input_rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for i in 0..input_rows[0].len() {
        let mut empty_col: Vec<char> = Vec::new();
        for j in 0..input_rows.len() {
            empty_col.push(input_rows[j][i]);
        }
        result.push(empty_col);
    }
    result
}

/*
fn pretty_print(input_rows: &Vec<Vec<char>>) {
    for row in input_rows {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}
*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}