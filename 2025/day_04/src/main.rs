use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_4.txt") {
        let grid = get_grid(file_iter);
        let result = process_grid(&grid);
        let mut grid_mut = grid.clone();
        let result2 = remove_and_process_grid(&mut grid_mut);
        println!("Result 1: {}", result);
        println!("Result 2: {}", result2);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn process_grid(grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    let mut top_row = vec![0; grid[0].len()];
    let mut second_row = grid[0].clone();
    for row_num in 1..grid.len() {
        let third_row = grid[row_num].clone();
        count += process_row(&top_row, &second_row, &third_row);
        top_row = second_row;
        second_row = third_row;
    }
    let third_row = vec![0; grid[0].len()];
    count += process_row(&top_row, &second_row, &third_row);
    count
}

fn remove_and_process_grid(mut grid: &mut Vec<Vec<u32>>) -> u32 {

    let mut count = 0;
    loop {
        let old_count = count;
        let mut top_row = vec![0; grid[0].len()];
        let mut second_row = grid[0].clone();
        for row_num in 1..grid.len() {
            let third_row = grid[row_num].clone();
            count += remove_and_process_row(&top_row, &second_row, &third_row, &mut grid, row_num);
            
            top_row = second_row;
            second_row = third_row;
        }
    
        let third_row = vec![0; grid[0].len()];
        let len = grid.len();
        count += remove_and_process_row(&top_row, &second_row, &third_row, &mut grid, len);
        if count == old_count {
            break;
        }
    }

    count
}

fn remove_and_process_row(top: &Vec<u32>, middle: &Vec<u32>, bottom: &Vec<u32>, grid: &mut Vec<Vec<u32>>, row_num: usize) -> u32 {
    let mut count = 0;
    for col in 0..middle.len() {
        let mut spot_count = 0;
        if middle[col] != 1 {
            continue;
        }
        if col > 0 && col < middle.len() - 1 {
            spot_count += top[col - 1] + top[col] + top[col + 1];
            spot_count += middle[col - 1] + middle[col + 1];
            spot_count += bottom[col - 1] + bottom[col] + bottom[col + 1];
        } else if col == 0 {
            spot_count += top[col] + top[col + 1];
            spot_count += middle[col + 1];
            spot_count += bottom[col] + bottom[col + 1];
        } else if col == middle.len() - 1 {
            spot_count += top[col - 1] + top[col];
            spot_count += middle[col - 1];
            spot_count += bottom[col - 1] + bottom[col];
        }
        if spot_count <= 3 {
            count += 1;
            grid[row_num-1][col] = 0;
        }
    }
    count
}

fn process_row(top: &Vec<u32>, middle: &Vec<u32>, bottom: &Vec<u32>) -> u32 {
    let mut count = 0;
    for col in 0..middle.len() {
        let mut spot_count = 0;
        if middle[col] != 1 {
            continue;
        }
        if col > 0 && col < middle.len() - 1 {
            spot_count += top[col - 1] + top[col] + top[col + 1];
            spot_count += middle[col - 1] + middle[col + 1];
            spot_count += bottom[col - 1] + bottom[col] + bottom[col + 1];
        } else if col == 0 {
            spot_count += top[col] + top[col + 1];
            spot_count += middle[col + 1];
            spot_count += bottom[col] + bottom[col + 1];
        } else if col == middle.len() - 1 {
            spot_count += top[col - 1] + top[col];
            spot_count += middle[col - 1];
            spot_count += bottom[col - 1] + bottom[col];
        }
        if spot_count <= 3 {
            count += 1;
        }
    }
    count
}

fn get_grid(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let row: Vec<u32> = ip.chars().map(|c| match c {
                '.' => 0,
                '@' => 1,
                _ => 0,
            }).collect();
            grid.push(row);
        }
    }
    grid
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let grid = get_grid(file_iter);
            let result = process_grid(&grid);
            assert_eq!(result, 13);
        }
    }
    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let mut grid = get_grid(file_iter);
            let result = remove_and_process_grid(&mut grid);
            assert_eq!(result, 43);
        }
    }
}
