use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(mut file_iter) = read_lines("input_9.txt") {
        let input_text = file_iter.next().unwrap();
        let tmp = process_input(input_text.unwrap());
        let total_1 = get_checksum(tmp.0.clone(), tmp.1.clone());
        let total_2 = get_checksum_2(tmp.0, tmp.1);
        println!("Total: {}", total_1);
        println!("Part 2: {}", total_2);
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

fn get_checksum_2(mut whites: Vec<Whitespace>, files: Vec<FileBlock>) -> usize {
    let mut my_files = files.clone();
    let mut checksum = 0;
    let mut remaining_whitespace = true;
    my_files.reverse();
    'place_file_block: for file_block in my_files.into_iter() {
        let mut file_copy = file_block.clone();
        if remaining_whitespace {
            'find_whitespace: for i in 0..whites.len() {
                loop {
                    match (whites[i].size > 0 && whites[i].size >= file_copy.size, file_copy.size > 0, file_copy.start_point > whites[i].start_point) {
                        (true, true, true) => {
                            checksum += file_copy.id * whites[i].start_point;
                            whites[i].start_point += 1;
                            whites[i].size -= 1;
                            file_copy.size -= 1;
                        },
                        (false, true, true) => {
                            continue 'find_whitespace;
                        },
                        (_, false, true) => {
                            continue 'place_file_block;
                        },
                        (_, _, false) => {
                            for i in 0..file_copy.size {
                                checksum += file_copy.id * (file_copy.start_point + i);
                            }
                            continue 'place_file_block;
                        }
                    };
                }
            }
            remaining_whitespace = false;
        }
        for i in 0..file_copy.size {
            checksum += file_copy.id * (file_copy.start_point + i);
        }

    }
    return checksum;
}

fn get_checksum(mut whites: Vec<Whitespace>, files: Vec<FileBlock>) -> usize {
    let mut my_files = files.clone();
    let mut checksum = 0;
    let mut remaining_whitespace = true;
    my_files.reverse();
    'place_file_block: for file_block in my_files.into_iter() {
        let mut file_copy = file_block.clone();
        if remaining_whitespace {
            'find_whitespace: for i in 0..whites.len() {
                loop {
                    match (whites[i].size > 0, file_copy.size > 0, file_copy.start_point > whites[i].start_point) {
                        (true, true, true) => {
                            checksum += file_copy.id * whites[i].start_point;
                            whites[i].start_point += 1;
                            whites[i].size -= 1;
                            file_copy.size -= 1;
                            
                        },
                        (false, true, true) => {
                            continue 'find_whitespace;
                        },
                        (_, false, true) => {
                            continue 'place_file_block;
                        },
                        (_, _, false) => {
                            break 'find_whitespace;
                        }
                    };
                }
            }
            remaining_whitespace = false;
        }
        for i in 0..file_copy.size {
            checksum += file_copy.id * (file_copy.start_point + i);
        }

    }
    return checksum;
}

fn process_input(input_text: String) -> (Vec<Whitespace>, Vec<FileBlock>) {
    let mut white: Vec<Whitespace> = Vec::new();
    let mut files: Vec<FileBlock> = Vec::new();
    let mut curr_id = 0;
    let mut curr_pos = 0;
    let mut arity = true;
    let mut char_enum = input_text.chars();
    'outer: loop {
        let new_size = char_enum.next().unwrap_or('a').to_digit(10);
        match new_size {
            Some(n) => {
                let usize_n: usize = n.try_into().unwrap();
                if arity {
                    files.push(FileBlock {
                        start_point: curr_pos,
                        size: usize_n,
                        id: curr_id,
                    });
                    curr_id += 1;
                    curr_pos += usize_n;
                } else {
                    white.push(Whitespace {
                        start_point: curr_pos,
                        size: usize_n,
                    });
                    curr_pos += usize_n;
                }
            }
            None => break 'outer,
        };
        arity = !arity;
        
    }
    return (white, files);
}

#[derive(Clone, Debug, PartialEq)]
struct Whitespace {
    start_point: usize,
    size: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct FileBlock {
    start_point: usize,
    size: usize,
    id: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let (vec_of_whitespaces, vec_of_files) =
                process_input(file_iter.next().unwrap().unwrap());
            assert_eq!(get_checksum(vec_of_whitespaces, vec_of_files), 1928);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(mut file_iter) = read_lines("test_input.txt") {
            let (vec_of_whitespaces, vec_of_files) = 
                process_input(file_iter.next().unwrap().unwrap());
            assert_eq!(get_checksum_2(vec_of_whitespaces, vec_of_files), 2858);
        }
    }

    #[test]
    fn test_process_input() {
        let input_string = String::from("12345");
        let whitespace_vec = vec![
            Whitespace{start_point: 1, size: 2},
            Whitespace{start_point: 6, size: 4},
        ];
        let file_vec = vec![
            FileBlock{ start_point: 0, size: 1, id: 0},
            FileBlock{ start_point: 3, size: 3, id: 1},
            FileBlock{ start_point: 10, size: 5, id: 2}
        ];
        assert_eq!(process_input(input_string), (whitespace_vec, file_vec));
    }
    
    #[test]
    fn small_test() {
        let whitespace_vec = vec![
            Whitespace{start_point: 1, size: 5},
        ];
        let file_vec = vec![
            FileBlock{ start_point: 0, size: 1, id: 0},
            FileBlock{ start_point: 6, size: 1, id: 1},
        ];
        assert_eq!(get_checksum(whitespace_vec, file_vec), 1);
        let whitespace_vec = vec![
            Whitespace{start_point: 1, size: 2},
            Whitespace{start_point: 6, size: 4},
        ];
        let file_vec = vec![
            FileBlock{ start_point: 0, size: 1, id: 0},
            FileBlock{ start_point: 3, size: 3, id: 1},
        ];
        assert_eq!(get_checksum(whitespace_vec, file_vec), 6);
    }
}
