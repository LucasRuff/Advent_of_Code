use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total: u64 = 0;
    if let Ok(mut file_iter) = read_lines("input_13.txt") {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut rows: Vec<Vec<char>> = Vec::new();
        loop {
            match file_iter.next() {
                Some(line) => {
                    let char_vec: Vec<char> = line.unwrap().chars().collect();
                    if char_vec.len() == 0 {
                        let cols = get_cols(&rows);
                        patterns.push(Pattern {rows, cols});
                        rows = Vec::new();
                    } else {
                        rows.push(char_vec.clone());
                    }
                },
                None => {
                    let cols = get_cols(&rows);
                    patterns.push(Pattern {rows, cols});
                    break; 
                },
            }
        }    
        'all_patterns: for current_pattern in patterns.iter() {
            let unsmudged_pattern_info;
            match find_mirror(current_pattern.rows.clone(), None) {
                Some(row) => { unsmudged_pattern_info = (0, row); },
                None => { 
                    match find_mirror(current_pattern.cols.clone(), None) {
                    Some(col) => { unsmudged_pattern_info = (1, col); },
                    None => { panic!("Error! no mirror found"); },
                    }
                },
            }
            for i in 0..(current_pattern.rows.len() * current_pattern.cols.len()) {
                let smudged_pattern = smudge(current_pattern, i);
                
                match find_mirror(smudged_pattern.rows.clone(), if unsmudged_pattern_info.0 == 0 {Some(unsmudged_pattern_info.1)} else {None}) {
                    Some(row) => {
                            total += 100 * row as u64; continue 'all_patterns;
                    },
                    None => { 
                        match find_mirror(smudged_pattern.cols.clone(), if unsmudged_pattern_info.0 == 1 {Some(unsmudged_pattern_info.1)} else {None}) {
                            Some(col) => {
                                    total += col as u64; continue 'all_patterns; 
                            },
                            None => { },
                        }
                    },
                }
            }
            panic!("Error: No mirror found");
        }
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn smudge(pat: &Pattern, smudge_index: usize) -> Pattern {
    let smudge_row = smudge_index / pat.cols.len();
    let smudge_col = smudge_index % pat.cols.len();
    let smudged_char = pat.rows[smudge_row][smudge_col];
    let mut rows_copy = pat.rows.clone();
    let mut cols_copy = pat.cols.clone();
    match smudged_char {
        '#' => {
            rows_copy[smudge_row][smudge_col] = '.';
            cols_copy[smudge_col][smudge_row] = '.';
        },
        '.' => {
            rows_copy[smudge_row][smudge_col] = '#';
            cols_copy[smudge_col][smudge_row] = '#';
        },
        _ => {},
    }
    return Pattern{rows: rows_copy, cols: cols_copy};
}

fn find_mirror(vecs: Vec<Vec<char>>, original_mirror: Option<usize>) -> Option<usize> {
    'outer: for i in 1..(vecs.len()) {
        'inner: for j in 0..i {
            if i+j >= vecs.len() { break 'inner; }
            if vecs[i-j-1] != vecs[i+j] {
                continue 'outer;
            }
        } 
        match original_mirror {
            Some(loc) => {
                if loc == i {
                    continue 'outer;
                } else {
                    return Some(i);
                }
            },
            None => { return Some(i); },
        }        
    }
    return None;
}

fn get_cols(input_rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Pattern {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}