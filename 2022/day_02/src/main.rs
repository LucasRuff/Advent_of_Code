use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let unwrapped_line = &line.unwrap()[..];
        let opponent_throw = unwrapped_line.chars().nth(0).unwrap();
        let outcome = unwrapped_line.chars().nth(2).unwrap();
        let my_throw = get_throw(opponent_throw, outcome);
        score = score + calculate_throw_score(my_throw);
        score = score + calculate_win_score(outcome);
    }
    println!("{}", score);
    Ok(())
}

fn calculate_throw_score(throw :char) -> i32 {
    match throw {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0
    }
}

fn get_throw(opp_throw :char, outcome :char) -> char {
    match (opp_throw, outcome) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        (_, _) => ' '
    }
}

fn calculate_win_score(outcome :char) -> i32 {
    match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0
    }
}