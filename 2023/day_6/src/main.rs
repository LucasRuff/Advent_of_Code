use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re_number: regex::Regex = Regex::new(r"[0-9]+").unwrap();
    if let Ok(mut file_iter) = read_lines("input_6.txt") {
        let times: Vec<String> = re_number.find_iter(&(file_iter.next().unwrap().unwrap())).map(|m| m.as_str().to_owned()).collect();
        let records: Vec<String> = re_number.find_iter(&(file_iter.next().unwrap().unwrap())).map(|m| m.as_str().to_owned()).collect();
        let mut big_time_text = "".to_owned();
        let mut big_record_text = "".to_owned();
        let mut num_victories: Vec<u64> = vec![0; times.len()];
        //let part_1 = false;
        /*
        if part_1 {
            for race_num in 0..times.len() {
                for hold_time in 0..times[race_num] {
                    if get_distance(hold_time, times[race_num]) > records[race_num] {
                        num_victories[race_num] += 1;
                    }
                }
            }
        } else {*/
            for race_num in 0..times.len() {
                big_time_text = big_time_text + &times[race_num];
                big_record_text = big_record_text + &records[race_num];
            }
            println!("Real race is {} millis, {} record", big_time_text, big_record_text);
            let big_time = big_time_text.parse::<u64>().unwrap();
            let big_record = big_record_text.parse::<u64>().unwrap();
            println!("Real race is {} millis, {} record", big_time, big_record);
            for hold_time in 0..big_time {
                if get_distance(hold_time, big_time) > big_record {
                    num_victories[0] += 1;
                }
            }
        //}
/*
        let mut total = 1;
        for race in num_victories {
            total *= race;
        }
        println!("total: {}", total);
        */
        println!("Total: {}", num_victories[0]);
    }
}

fn get_distance(hold: u64, total: u64) -> u64 {
    let speed = hold;
    let travel_time = total - hold;
    return speed * travel_time;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}