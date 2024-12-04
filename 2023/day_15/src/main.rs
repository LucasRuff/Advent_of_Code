use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut total: usize = 0;
    if let Ok(file_iter) = read_lines("input_15_test.txt") {
        for line in file_iter {
            let mut box_vec: Vec<LensBox> = Vec::new();
            for _ in 0..256 {
                box_vec.push(LensBox { lenses: Vec::new() });
            }
            if let Ok(text) = line {
                let instruction_iter = text.split(',');
                for instruction in instruction_iter {
                    let operator_posn = instruction.chars().position(|c| {c=='=' || c=='-'}).unwrap();
                    let label = &instruction[0..operator_posn];
                    
                    let box_num = hash_chars(&instruction[0..operator_posn]) as usize;
                    match instruction.chars().nth(operator_posn) {
                        Some('-') => {
                            if let Some(pos) = box_vec[box_num].lenses.iter().position(|x| x.label == label) {
                                box_vec[box_num].lenses.remove(pos);
                            }
                        },
                        Some('=') => {
                            if let Some(pos) = box_vec[box_num].lenses.iter().position(|x| x.label == label) {
                                box_vec[box_num].lenses[pos].focal_length = instruction.chars().nth(operator_posn + 1).unwrap();
                            } else {
                                box_vec[box_num].lenses.push(Lens { label: label.to_string(), focal_length: instruction.chars().nth(operator_posn + 1).unwrap() });
                            }
                        },
                        Some(k) => { panic!("Error: could not read instruction {}", k); },
                        None => { panic!("Error: no instruction found") }
                    }
                }
            }
            for (i, lens_box) in box_vec.iter().enumerate() {
                for (j, lens) in lens_box.lenses.iter().enumerate() {
                    total += (1+i) * (j+1) * lens.focal_length.to_string().parse::<usize>().unwrap();
                }
            }
        }
    }
    println!("{}", total);
    println!("Finished in {:?}", now.elapsed());
}

fn hash_chars(instruction: &str) -> u64 {
    let char_iter = instruction.chars();
    let mut total = 0;
    for cha in char_iter {
        total += cha as u64;
        total *= 17;
        total %= 256;
    }
    total
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Lens {
    label: String,
    focal_length: char,
}

struct LensBox {
    lenses: Vec<Lens>,
}