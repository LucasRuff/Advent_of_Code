use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let lines_iter = get_line_iter("input.txt").unwrap().lines();
    let mut answer_counter = 0;
    
    for line in lines_iter {
        let unwrapped_line = line.unwrap();
        let mut elf_one = Elf::new();
        let mut elf_two = Elf::new();
        (elf_one, elf_two) = get_elf_sections(elf_one, elf_two, unwrapped_line);
        if any_overlap(elf_one, elf_two) {answer_counter = answer_counter + 1;}
    }
    println!("{answer_counter}");
}

fn get_line_iter(filename :&str) -> Option<std::io::BufReader<std::fs::File>> {
    let file = File::open(filename).ok()?;
    Some(BufReader::new(file))
}

fn get_elf_sections(mut elf_one :Elf, mut elf_two :Elf, line :String) -> (Elf, Elf) {
    let line_sections = &line[..];
    let mut line_sections_clone = line_sections.clone().split(&['-',',']);
    elf_one.first_section = line_sections_clone.next().unwrap().parse().unwrap();
    elf_one.last_section = line_sections_clone.next().unwrap().parse().unwrap();
    elf_two.first_section = line_sections_clone.next().unwrap().parse().unwrap();
    elf_two.last_section = line_sections_clone.next().unwrap().parse().unwrap();
    (elf_one, elf_two)
}

fn fully_contains(elf_one :Elf, elf_two :Elf) -> bool {
    if elf_one.first_section <= elf_two.first_section && elf_one.last_section >= elf_two.last_section {return true}
    else if elf_two.first_section <= elf_one.first_section && elf_two.last_section >= elf_one.last_section {return true}
    else {return false}
}

fn any_overlap(elf_one :Elf, elf_two :Elf) -> bool {
    if elf_one.first_section <= elf_two.last_section && elf_one.last_section >= elf_two.first_section {return true}
    else {return false}
}

pub struct Elf {
    pub first_section: i32,
    pub last_section: i32
}

impl Elf {
    pub fn new() -> Self {
        Self {
            first_section: 0,
            last_section: 0
        }
    }
}