use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total_priority = 0;
/*
    for line in reader.lines() {
        let unwrapped_line = &line.unwrap()[..];
        let compartment_size = unwrapped_line.len() / 2;
        let compartment_one = &unwrapped_line[..compartment_size];
        let compartment_one_iter = compartment_one.clone().chars();
        let mut similarity_found = false;
        for item_one in compartment_one_iter {
            let compartment_two = &unwrapped_line[compartment_size..];
            let compartment_two_iter = compartment_two.clone().chars();
            for item_two in compartment_two_iter {
                if item_one == item_two {
                    total_priority = total_priority + get_priority(item_one);
                    similarity_found = true;
                }
                if similarity_found {break};
            }
            if similarity_found {break};
        }


    }
    println!("{}", total_priority);
*/
    let mut line_iter = reader.lines();
    loop {
        match line_iter.next() {
            Some(s) => {let elf_1 = s.unwrap();
                let elf_2 = line_iter.next().unwrap().unwrap();
                let elf_3 = line_iter.next().unwrap().unwrap();
                let initial_share = get_shared(&elf_1[..], &elf_2[..]);
                let badge = get_shared(&initial_share.iter().collect::<String>(), &elf_3[..]);
                total_priority = total_priority + get_priority(badge[0]);
            }
            None => break
        };

    }
    println!("{total_priority}");

    Ok(())
}

fn get_priority(item :char) -> usize {
    let priority_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    priority_list.chars().position(|x| x == item).unwrap() + 1
}

fn get_shared(str_one :&str, str_two :&str) -> Vec<char> {
    let mut shared = Vec::new();
    for char_one in str_one.chars() {
        for char_two in str_two.chars() {
            if char_one == char_two {shared.push(char_one);}
        }
    }
    shared.sort();
    shared.dedup();
    shared
}