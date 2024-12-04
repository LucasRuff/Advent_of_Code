use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let mut total = 0;
    let part_1 = false;
    let re_number: Regex = Regex::new(r"[0-9]+").unwrap();
    let re_gear: Regex = Regex::new(r"\*").unwrap();
    let file_size = read_lines("input_3.txt").unwrap().count();
    println!("{} lines in file", file_size);
    if let Ok(mut file_iter) = read_lines("input_3.txt") {
        let Some(first_line_res) = file_iter.next() else {panic!()};
        let Some(second_line_res) = file_iter.next() else {panic!()};
        let mut first_line = ".".to_owned() + &first_line_res.unwrap() + ".";
        let mut second_line = ".".to_owned() + &second_line_res.unwrap() + ".";
        for (line_num, line) in file_iter.enumerate() {
            let mut third_line = line.unwrap();
            third_line = ".".to_owned() + &third_line + ".";
            if part_1 {
                for number_in_line in re_number.find_iter(&second_line) {
                    let number_position = number_in_line.start();
                    let upper_neighborhood = &first_line[number_position-1..number_in_line.end()+1];
                    let lower_neighborhood = &third_line[number_position-1..number_in_line.end()+1];
                    let neighborhood = upper_neighborhood.to_owned() + lower_neighborhood + &second_line.as_str()[number_position-1..number_in_line.end()+1];
                    for neighborhood_char in neighborhood.chars() {
                        if !neighborhood_char.is_digit(10) && neighborhood_char != '.' {
                            total += number_in_line.as_str().parse::<u32>().unwrap();
                            break;
                        }
                    }
                }

                if line_num == file_size-3 {
                    for number_in_line in re_number.find_iter(&third_line) {
                        let number_position = number_in_line.start();
                        let upper_neighborhood = &second_line[number_position-1..number_in_line.end()+1];
                        let neighborhood = upper_neighborhood.to_owned() + &third_line.as_str()[number_position-1..number_in_line.end()+1];
                        for neighborhood_char in neighborhood.chars() {
                            if !neighborhood_char.is_digit(10) && neighborhood_char != '.' {
                                total += number_in_line.as_str().parse::<u32>().unwrap();
                                break;
                            }
                        }
                    }
                } else if line_num == 0 {
                    for number_in_line in re_number.find_iter(&first_line) {
                        let number_position = number_in_line.start();
                        let lower_neighborhood = &second_line[number_position-1..number_in_line.end()+1];
                        let neighborhood = lower_neighborhood.to_owned() + &first_line.as_str()[number_position-1..number_in_line.end()+1];
                        for neighborhood_char in neighborhood.chars() {
                            if !neighborhood_char.is_digit(10) && neighborhood_char != '.' {
                                total += number_in_line.as_str().parse::<u32>().unwrap();
                                break;
                            }
                        }
                    }
                }
            } else {
                    for gear_in_line in re_gear.find_iter(&second_line) {
                    let mut gear_ratio_count = 0;
                    let mut gear_ratio = 1;
                    let gear_position = gear_in_line.start();
                    let upper_neighborhood = &first_line[gear_position-3..gear_position+4];
                    let lower_neighborhood = &third_line[gear_position-3..gear_position+4];
                    let side_neighborhood = &second_line.as_str()[gear_position-3..gear_position+4];
                    //println!("Found gear on line {}, position {}", line_num, gear_position);
                    for input in re_number.find_iter(&upper_neighborhood) {
                        //println!("Input: {}", input.as_str());
                        if input.start() < 5 && input.end() > 2 {
                            gear_ratio *= input.as_str().parse::<u32>().unwrap();
                            gear_ratio_count += 1;
                        }
                    }  
                    for input in re_number.find_iter(&lower_neighborhood) {
                        if input.start() <5 && input.end() > 2 {
                            gear_ratio *= input.as_str().parse::<u32>().unwrap();
                            gear_ratio_count += 1;
                        }
                    }
                    for input in re_number.find_iter(&side_neighborhood) {
                        if input.start() < 5 && input.end() > 2 {
                            gear_ratio *= input.as_str().parse::<u32>().unwrap();
                            gear_ratio_count += 1;
                        }
                    }
                    if gear_ratio_count == 2 {
                        //println!("Found gear with ratio {} on line {}", gear_ratio, line_num);
                        total += gear_ratio;
                    }
                }
                if line_num == file_size - 3 {
                    for gear_in_line in re_gear.find_iter(&third_line) {
                        let mut gear_ratio_count = 0;
                        let mut gear_ratio = 1;
                        let gear_position = gear_in_line.start();
                        let upper_neighborhood = &second_line[gear_position-3..gear_position+4];
                        let side_neighborhood = &third_line.as_str()[gear_position-3..gear_position+4];
                        //println!("Found gear on line {}, position {}", line_num, gear_position);
                        for input in re_number.find_iter(&upper_neighborhood) {
                            //println!("Input: {}", input.as_str());
                            if input.start() < 5 && input.end() > 2 {
                                gear_ratio *= input.as_str().parse::<u32>().unwrap();
                                gear_ratio_count += 1;
                            }
                        }  
                        for input in re_number.find_iter(&side_neighborhood) {
                            if input.start() < 5 && input.end() > 2 {
                                gear_ratio *= input.as_str().parse::<u32>().unwrap();
                                gear_ratio_count += 1;
                            }
                        }
                        if gear_ratio_count == 2 {
                            //println!("Found gear with ratio {} on line {}", gear_ratio, line_num);
                            total += gear_ratio;
                        }
                    }
                } else if line_num == 0 {
                    for gear_in_line in re_gear.find_iter(&first_line) {
                        let mut gear_ratio_count = 0;
                        let mut gear_ratio = 1;
                        let gear_position = gear_in_line.start();
                        let lower_neighborhood = &second_line[gear_position-3..gear_position+4];
                        let side_neighborhood = &first_line.as_str()[gear_position-3..gear_position+4];
                        //println!("Found gear on line {}, position {}", line_num, gear_position);
                        for input in re_number.find_iter(&lower_neighborhood) {
                            if input.start() <5 && input.end() > 2 {
                                gear_ratio *= input.as_str().parse::<u32>().unwrap();
                                gear_ratio_count += 1;
                            }
                        }
                        for input in re_number.find_iter(&side_neighborhood) {
                            if input.start() < 5 && input.end() > 2 {
                                gear_ratio *= input.as_str().parse::<u32>().unwrap();
                                gear_ratio_count += 1;
                            }
                        }
                        if gear_ratio_count == 2 {
                            //println!("Found gear with ratio {} on line {}", gear_ratio, line_num);
                            total += gear_ratio;
                        }
                    }
                }
            }
            first_line = second_line;
            second_line = third_line;
        }
    }
    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}