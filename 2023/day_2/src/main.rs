use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
// 12 red, 13 green, 14 blue

fn main() {
    let mut total = 0;
    if let Ok(file_iter) = read_lines("input_2.txt") {
        for (_, line) in file_iter.enumerate() {
            let good_line = line.unwrap();
            let _game_num = get_game_num(&good_line);
            let game_iter = get_game_iter(&good_line);
            //let mut impossible_rounds = 0;
            let mut min_cubes = [0, 0, 0];
            for round in game_iter {
                /*
                if round.red > 12 || round.green > 13 || round.blue > 14 {
                    impossible_rounds += 1;
                }
                 */
                if round.red > min_cubes[0] {min_cubes[0] = round.red};
                if round.blue > min_cubes[1] {min_cubes[1] = round.blue};
                if round.green > min_cubes[2] {min_cubes[2] = round.green};
            }
            /*
            if impossible_rounds > 0 {total += game_num};
             */
            total += min_cubes[0] * min_cubes[1] * min_cubes[2];
        }
    }
    println!("{}", total);
}

fn get_game_num(game: &str) -> u32 {
    let re_gamenum: regex::Regex = Regex::new(r"Game [0-9]+").unwrap();
    let mut game_num = 0;
    let Some(game_number) = re_gamenum.captures(game) else {return 0};
    for chas in game_number[0].chars() {
        if chas.is_digit(10) {
            game_num = game_num * 10 + chas.to_string().parse::<u32>().unwrap();
        }
    }
    return game_num;
}

fn get_game_iter(game: &str) -> Vec<Round> {
    let re_reds: regex::Regex = Regex::new(r"[0-9]+ red").unwrap();
    let re_blues: regex::Regex = Regex::new(r"[0-9]+ blue").unwrap();
    let re_greens: regex::Regex = Regex::new(r"[0-9]+ green").unwrap();
    let mut game_iter: Vec<Round> = Vec::new();
    let start_point = game.find(":").unwrap();
    let rounds_iter = game[start_point+1..].split(";");
    for (_, round) in rounds_iter.enumerate() {
        let mut new_round: Round = Round { red: 0, green: 0, blue: 0 };
        new_round.red = match re_reds.captures(round) {
            Some(n) => n[0][0..2].trim().parse::<u32>().unwrap(),
            None => 0,
        };
        new_round.blue = match re_blues.captures(round) {
            Some(n) => n[0][0..2].trim().parse::<u32>().unwrap(),
            None => 0,
        };
        new_round.green = match re_greens.captures(round) {
            Some(n) => n[0][0..2].trim().parse::<u32>().unwrap(),
            None => 0,
        };
        game_iter.push(new_round);
    }
    return game_iter;
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}