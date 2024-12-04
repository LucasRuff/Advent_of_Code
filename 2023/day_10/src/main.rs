use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let mut total: u64 = 0;
    //let re_line_parse: regex::Regex = Regex::new(r"[0-9A-Z]+").unwrap();
    let mut file_vec: Vec<Vec<char>> = Vec::new();
    if let Ok(file_iter) = read_lines("input_10.txt") {
        for line in file_iter {
            if let Ok(text) = line {
                file_vec.push(text.chars().collect());
            }
        }
        let mut start_pos: (usize, usize) = (0,0);
        for i in 0..file_vec.len()-1 {
            for j in 0..file_vec[0].len()-1 {
                if file_vec[i][j] == 'S' { start_pos = (i,j);}
            }
        }
        let mut current_pos: (usize, usize) = (0,0);
        let mut last_direction = '*';
        let mut path: Vec<(usize, usize, char)> = Vec::new();

        total += 1;
        if start_pos.0 != 0 {
            let up = file_vec[start_pos.0 - 1][start_pos.1];
            if up == '|' || up == '7' || up == 'F' {
                current_pos = (start_pos.0 - 1, start_pos.1);
                last_direction = 'D';
                path.push((start_pos.0, start_pos.1, '*'));
            }
        }
        if start_pos.0 != file_vec.len()-1 {
            let down = file_vec[start_pos.0 + 1][start_pos.1];
            if down == '|' || down == 'J' || down == 'L' {
                current_pos = (start_pos.0 + 1, start_pos.1);
                last_direction = 'U';
                path.push((start_pos.0, start_pos.1, '*'));
            }
        }
        if start_pos.1 != 0 {
            let left = file_vec[start_pos.0][start_pos.1 - 1];
            if left == '-' || left == 'F' || left == 'L' {
                current_pos = (start_pos.0, start_pos.1 - 1);
                last_direction = 'R';
                path.push((start_pos.0, start_pos.1, '*'));
            }
        }
        if start_pos.1 != file_vec[0].len()-1 {
            let right = file_vec[start_pos.0][start_pos.1 + 1];
            if right == '-' || right == 'J' || right == '7' {
                current_pos = (start_pos.0, start_pos.1 + 1);
                last_direction = 'L';
                path.push((start_pos.0, start_pos.1, '*'));
            }
        }
        let mut fill_map: Vec<Vec<char>> = vec![vec!['*'; file_vec[0].len() + 1]; file_vec.len() + 1];
        'bigloop: loop {
            total += 1;
            //println!("{}  at step {}, position {}, {}", file_vec[current_pos.0][current_pos.1], total, current_pos.0, current_pos.1);
            path.push((current_pos.0, current_pos.1, last_direction));
            fill_map[current_pos.0][current_pos.1] = 'P';
            
            match file_vec[current_pos.0][current_pos.1] {
                '|' => {
                    if last_direction == 'U' {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0 + 1, current_pos.1);
                        
                    } else {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0 - 1, current_pos.1);

                    }
                }, 
                '-' => {
                    if last_direction == 'L' {
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0, current_pos.1 + 1);
                    } else {
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0, current_pos.1 - 1);
                    }
                },
                'L' => {
                    if last_direction == 'U' {
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 + 1][current_pos.1 - 1] = if fill_map[current_pos.0 + 1][current_pos.1 - 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0, current_pos.1 + 1);
                        last_direction = 'L';
                    } else {
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 + 1][current_pos.1 - 1] = if fill_map[current_pos.0 + 1][current_pos.1 - 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0 - 1, current_pos.1);
                        last_direction = 'D';
                    }
                },
                'J' => {
                    if last_direction == 'U' {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 + 1][current_pos.1 + 1] = if fill_map[current_pos.0 + 1][current_pos.1 + 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0, current_pos.1 - 1);
                        last_direction = 'R';
                    } else {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 + 1][current_pos.1 + 1] = if fill_map[current_pos.0 + 1][current_pos.1 + 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 + 1][current_pos.1] = if fill_map[current_pos.0 + 1][current_pos.1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0 - 1, current_pos.1);
                        last_direction = 'D';
                    }
                },
                '7' => {
                    if last_direction == 'D' {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 - 1][current_pos.1 + 1] = if fill_map[current_pos.0 - 1][current_pos.1 + 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0, current_pos.1 - 1);
                        last_direction = 'R';
                    } else {
                        fill_map[current_pos.0][current_pos.1 + 1] = if fill_map[current_pos.0][current_pos.1 + 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 - 1][current_pos.1 + 1] = if fill_map[current_pos.0 - 1][current_pos.1 + 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0 + 1, current_pos.1);
                        last_direction = 'U';
                    }
                },
                'F' => {
                    if last_direction == 'D' {
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 - 1][current_pos.1 - 1] = if fill_map[current_pos.0 - 1][current_pos.1 - 1] == 'P' {'P'} else {'L'};
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'L'};
                        current_pos = (current_pos.0, current_pos.1 + 1);
                        last_direction = 'L';
                    } else {
                        fill_map[current_pos.0][current_pos.1 - 1] = if fill_map[current_pos.0][current_pos.1 - 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 - 1][current_pos.1 - 1] = if fill_map[current_pos.0 - 1][current_pos.1 - 1] == 'P' {'P'} else {'R'};
                        fill_map[current_pos.0 - 1][current_pos.1] = if fill_map[current_pos.0 - 1][current_pos.1] == 'P' {'P'} else {'R'};
                        current_pos = (current_pos.0 + 1, current_pos.1);
                        last_direction = 'U';
                    }
                },
                'S' => {
                    println!("Circuit Complete!");
                    break 'bigloop;
                },
                _ => {panic!("dead end");}
            }
        }
        let mut changes = 1;
        while changes != 0 {
            changes = 0;
            for i in 0..fill_map.len()-1 {
                for j in 0..fill_map[0].len()-1 {
                    if fill_map[i][j] == 'L' {
                        if fill_map[i+1][j] == '*' {fill_map[i+1][j] = 'L'; changes += 1;}
                        if fill_map[i-1][j] == '*' {fill_map[i-1][j] = 'L'; changes += 1;}
                        if fill_map[i][j-1] == '*' {fill_map[i][j-1] = 'L'; changes += 1;}
                        if fill_map[i][j+1] == '*' {fill_map[i][j+1] = 'L'; changes += 1;}
                    }
                }
            }
        }

        let mut num_inside = 0;

        for line in fill_map {
            //let row: String = line.iter().collect();
            //println!("{}", row);
            for character in line {
                if character == 'L' {
                    num_inside += 1;
                }
            }
        }
        println!("Enclosed space: {}", num_inside);
    }
    println!("{}", total/2);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}