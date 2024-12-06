use std::io::{BufRead, BufReader, self};
use std::fs::File;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut input_buffer = BufReader::new(file);
    let mut input_string = String::new();
    input_buffer.read_line(&mut input_string).expect("Failed to read line\n");
    let mut input_bytes = input_string.into_bytes();
    input_bytes.reverse();
    //let packet_start_bytes = start_of_packet(input_bytes);
    let message_start_bytes = first_n_unique(input_bytes, 14);
    println!("{message_start_bytes}");


    Ok(())
}

/*
fn start_of_packet(mut bytestream :Vec<u8>) -> usize {
    let mut chars_read = 4;
    let mut frontier: VecDeque<u8> = VecDeque::new();
    for _ in 0..4 {
        frontier.push_back(bytestream.pop().unwrap());
    }
    let mut found_match_flag = true;
    let mut matched_char_number = 1;
    loop {
        let c = match bytestream.pop() {
            Some(s) => s,
            None => panic!(),
        };

        println!("Frontier: {:?}", frontier);
        println!("Character read: {c}");
        for i in 1..4 {
            if c == frontier[i] {
                found_match_flag = true;
                matched_char_number = i;
            }
        }
        frontier.pop_front();
        frontier.push_back(c);
        println!("Matched char #: {matched_char_number}");
        if !found_match_flag {return chars_read}
        chars_read = chars_read + 1;
        if matched_char_number == 0 {
            found_match_flag = false;
        } else {matched_char_number = matched_char_number - 1;}
    }
}
*/

fn first_n_unique(mut bytestream :Vec<u8>, n :usize) -> usize {
    let mut chars_read = 0;
    let mut frontier = VecDeque::new();
    let mut unique_flag = false;
    loop {
        if frontier.len() == n && unique_flag {return chars_read}
        
        let c = match bytestream.pop() {
            Some(s) => s,
            None => panic!(),
        };
        chars_read = chars_read + 1;

        frontier.push_back(c);
        if frontier.len() > n {
            frontier.pop_front();
        }
        unique_flag = true;
        for i in 0..frontier.len()-1 {
            for j in i+1..frontier.len() {
                if frontier[i] == frontier[j] {
                    //println!("In line {:?}:\n", frontier);
                    //println!("Matched {:?} to {:?}", frontier[i], frontier[j]);
                    unique_flag = false;
                }
            }
        }
    }
}