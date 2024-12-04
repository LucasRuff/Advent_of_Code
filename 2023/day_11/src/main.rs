use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total: i64 = 0;
    if let Ok(file_iter) = read_lines("input_11.txt") {
        let mut image: Vec<Vec<char>> = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                image.push(text.chars().collect());
            }
        }
        let (empty_rows, empty_cols) = get_empty_rows_cols(&image);
        //println!("{:?}", empty_rows);
        //println!("{:?}", empty_cols);
        //pretty_print(&image);
        //let expanded_image = expand_universe(&image, &empty_rows, &empty_cols);
        //pretty_print(&expanded_image);
        let expanded_galaxy_list = expand_galaxy_coordinates(&find_galaxies(&image), &empty_rows, &empty_cols);
        for i in 0..expanded_galaxy_list.len() {
            for j in 0..expanded_galaxy_list.len() {
                total += get_manhattan(expanded_galaxy_list[i], expanded_galaxy_list[j]);
            }
        }
    }
    println!("{}", total/2);
    println!("Finished in {:?}", now.elapsed());
}

fn get_manhattan(galaxy_a: (usize, usize), galaxy_b: (usize, usize)) -> i64 {
    let y_distance = (galaxy_a.0 as i64 - galaxy_b.0 as i64).abs();
    let x_distance = (galaxy_a.1 as i64 - galaxy_b.1 as i64).abs();
    return x_distance + y_distance;
}

fn pretty_print(universe: &Vec<Vec<char>>) -> () {
    for line in universe {
        let row: String = line.iter().collect();
        println!("{}", row);
    }
}

fn expand_galaxy_coordinates(galaxy_list: &Vec<(usize, usize)>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> Vec<(usize, usize)>{
    let mut result = Vec::new();
    for galaxy in galaxy_list {
        let mut num_expansions_x = 0;
        let mut num_expansions_y = 0;
        for row in empty_rows.iter() {
            if galaxy.0 > *row {
                num_expansions_y += 1;
            }
        }
        for col in empty_cols.iter() {
            if galaxy.1 > *col {
                num_expansions_x += 1;
            }
        }
        result.push((galaxy.0 + 999999*num_expansions_y, galaxy.1 + 999999*num_expansions_x));
    }
    return result;
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for row_num in 0..universe.len() {
        for col_num in 0..universe[0].len() {
            if universe[row_num][col_num] == '#' {
                result.push((row_num, col_num));
            }
        }
    }
    return result;
}
/*
fn expand_universe(universe: &Vec<Vec<char>>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> Vec<Vec<char>> {
    let mut final_expanded = Vec::new();
    let mut next_row = 0;
    for row_num in 0..universe.len() {
        let mut next_col = 0;
        let mut row_to_push = Vec::new();
        for col_num in 0..universe[0].len() {
            row_to_push.extend(vec!(universe[row_num][col_num]));
            if next_col < empty_cols.len() {
                if col_num == empty_cols[next_col] {
                    row_to_push.extend(vec!('.'));
                    next_col += 1;
                }
            }
        }
        final_expanded.push(row_to_push.clone());
        if next_row < empty_rows.len() {
            if row_num == empty_rows[next_row] {
            final_expanded.push(row_to_push.clone());
            next_row += 1;
            }
        }
    }
    return final_expanded;
}
*/
fn get_empty_rows_cols(image: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    'row_loop: for line in 0..image.len() {
        for cha_num in 0..image[0].len() {
            if image[line][cha_num] == '#' {
                continue 'row_loop;
            }
        }
        empty_rows.push(line);
    }
    'col_loop: for col in 0..image[0].len()-1 {
        for cha_num in 0..image.len() {
            if image[cha_num][col] == '#' {
                continue 'col_loop;
            }
        }
        empty_cols.push(col);
    }
    return (empty_rows, empty_cols);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}