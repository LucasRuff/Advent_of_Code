use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let re_number: regex::Regex = Regex::new(r"[0-9]+").unwrap();
    let re_map_start: regex::Regex = Regex::new(r"map").unwrap();
    //let mut minimum = u64::MAX;
    let mut map_array: [Vec<Mapping>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut ans_vec: Vec<u64> = Vec::new();
    if let Ok(mut file_iter) = read_lines("input_5.txt") {
        let Some(seeds_line) = file_iter.next() else {panic!()};
        //let part_1 = false;
        let mut seed_numbers: Vec<SeedRange> = Vec::new();

       // if part_1 {
        //    seed_numbers = re_number.find_iter(&(seeds_line.unwrap())).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
        //} else {
            let seed_line_nums: Vec<u64> = re_number.find_iter(&(seeds_line.unwrap())).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
            let mut seed_pair = 0;
            let mut start_seed: u64 = 0;
            for seedling in seed_line_nums {
                if seed_pair == 0 {
                    start_seed = seedling;
                    seed_pair = 1;
                } else {
                    seed_numbers.push(SeedRange{ start_seed: start_seed, end_seed: seedling + start_seed - 1, depth: 0});
                    seed_pair = 0;
                }
            }
            println!("Seeds created");
        //}

        //let seed_locations: Vec<u64> = Vec::new();

        let mut map_index: usize = 0;
        for line in file_iter {
            if let Ok(text) = line {
                if text.chars().next().is_none() {
                    continue;
                } else if re_map_start.find(&text).is_some() {
                    map_index += 1;
                    //println!("Map # {}", map_index);
                } else {
                    let nums_in_line: Vec<u64> = re_number.find_iter(&text).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
                    //println!("Found nums {:?}", nums_in_line);
                    let start_first = nums_in_line[0];
                    let start_second = nums_in_line[1];
                    let end_first = start_first + nums_in_line[2] - 1;
                    let end_second = start_second + nums_in_line[2] - 1;
                    map_array[map_index-1].push(Mapping{start_from: start_second, end_from: end_second, start_to: start_first, end_to: end_first});
                    //println!("New mapping: {} to {} maps to {} to {}", start_first, end_first, start_second, end_second);
                }
            }
        }
        let mut _seed_range = SeedRange{start_seed: u64::MAX, end_seed: u64::MAX, depth: 0};
        while let Some(mapped_seed) = seed_numbers.pop() {
            //println!("New mapped seed: {}, {}, {}", mapped_seed.start_seed, mapped_seed.end_seed, mapped_seed.depth);
            /*if part_1 {
                let mut inter_val = seed;
                for map_type in &map_array {
                    let mut mapped = false;
                    for map in map_type {
                        if mapped {continue;}
                        if inter_val >= map.start_from && inter_val <= map.end_from {
                            let new_val = map.start_to + (inter_val - map.start_from);

                            inter_val = new_val;
                            mapped = true;
                        }
                    }
                }
                seed_locations.push(inter_val);
            } else {*/
            if mapped_seed.depth == 7 {
                ans_vec.push(mapped_seed.start_seed);
                //println!("Found location: {}", mapped_seed.start_seed);
                continue;
            } else {
                let mut shelf: Vec<SeedRange> = vec!(mapped_seed);
                while let Some(seed_range) = shelf.pop() {
                    let mut mapped = false;
                    for current_map in &map_array[seed_range.depth as usize] {
                        if !mapped {
                            if seed_range.start_seed >= current_map.start_from && seed_range.end_seed <= current_map.end_from {
                                seed_numbers.push(SeedRange{start_seed: (seed_range.start_seed - current_map.start_from) + current_map.start_to, end_seed: (seed_range.end_seed - current_map.start_from) + current_map.start_to, depth: seed_range.depth + 1});
                                mapped = true;
                            } else if seed_range.start_seed < current_map.start_from && seed_range.end_seed > current_map.end_from {
                                seed_numbers.push(SeedRange{start_seed: current_map.start_to, end_seed: current_map.end_to, depth: seed_range.depth + 1});
                                shelf.push(SeedRange{start_seed: seed_range.start_seed, end_seed: current_map.start_from - 1, depth: seed_range.depth});
                                shelf.push(SeedRange{start_seed: current_map.end_from + 1, end_seed: seed_range.end_seed, depth: seed_range.depth});
                                mapped = true;
                            } else if seed_range.start_seed >= current_map.start_from && seed_range.start_seed <= current_map.end_from && seed_range.end_seed > current_map.end_from {
                                seed_numbers.push(SeedRange{start_seed: current_map.start_to + (seed_range.start_seed - current_map.start_from), end_seed: current_map.end_to, depth: seed_range.depth + 1});
                                shelf.push(SeedRange{start_seed: current_map.end_from + 1, end_seed: seed_range.end_seed, depth: seed_range.depth});
                                mapped = true;
                            } else if seed_range.start_seed < current_map.start_from && seed_range.end_seed <= current_map.end_from && seed_range.end_seed >= current_map.start_from {
                                seed_numbers.push(SeedRange{start_seed: current_map.start_to, end_seed: (seed_range.end_seed - current_map.start_from) + current_map.start_to, depth: seed_range.depth + 1});
                                shelf.push(SeedRange{start_seed: seed_range.start_seed, end_seed: current_map.start_from - 1, depth: seed_range.depth});
                                mapped = true;
                            }
                        }
                    }
                    if !mapped {
                        seed_numbers.push(SeedRange{start_seed: seed_range.start_seed, end_seed: seed_range.end_seed, depth: seed_range.depth + 1});
                    }
                }
            }
        }
        let minimum = ans_vec.iter().min().unwrap();
        println!("Min: {}", minimum);
        println!("Finished in {:?}", now.elapsed());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Mapping {
    start_from: u64,
    end_from: u64,
    start_to: u64,
    end_to: u64,
}

struct SeedRange {
    start_seed: u64,
    end_seed: u64,
    depth: u64,
}