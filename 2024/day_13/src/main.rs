use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::num::ParseIntError;
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_13.txt") {
        let Ok(machines) = get_games(file_iter) else {
            panic!("unable to parse input")
        };
        let mut total_1 = 0;
        let mut total_2 = 0;
        for machine in machines {
            total_1 += win_game_cost(&machine, false).unwrap_or(0);
            total_2 += win_game_cost(&machine, true).unwrap_or(0);
        }
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

fn get_games(i_file: io::Lines<io::BufReader<File>>) -> Result<Vec<Machine>, ParseIntError> {
    let a_regex: Regex = Regex::new(r"A: X\+([0-9]+)\, Y\+([0-9]+)").unwrap();
    let b_regex: Regex = Regex::new(r"B: X\+([0-9]+)\, Y\+([0-9]+)").unwrap();
    let prize_regex: Regex = Regex::new(r"Prize: X\=([0-9]+)\, Y\=([0-9]+)").unwrap();
    let mut a_vals: Vec<Button> = Vec::new();
    let mut b_vals: Vec<Button> = Vec::new();
    let mut prize_vals: Vec<(isize, isize)> = Vec::new();
    let all_lines: String = i_file.map(|line| line.unwrap()).collect();
    for (_, [x_val, y_val]) in a_regex.captures_iter(&all_lines).map(|c| c.extract()) {
        a_vals.push(Button {
            x: x_val.parse::<isize>()?,
            y: y_val.parse::<isize>()?,
        });
    }
    for (_, [x_val, y_val]) in b_regex.captures_iter(&all_lines).map(|c| c.extract()) {
        b_vals.push(Button {
            x: x_val.parse::<isize>()?,
            y: y_val.parse::<isize>()?,
        });
    }
    for (_, [x_val, y_val]) in prize_regex.captures_iter(&all_lines).map(|c| c.extract()) {
        prize_vals.push((x_val.parse::<isize>()?, y_val.parse::<isize>()?));
    }
    let machine_vec: Vec<Machine> = zip(zip(a_vals, b_vals), prize_vals)
        .map(|((a_but, b_but), (x_goal, y_goal))| Machine {
            a: a_but,
            b: b_but,
            prize_x: x_goal,
            prize_y: y_goal,
        })
        .collect();
    return Ok(machine_vec);
}

fn win_game_cost(machine: &Machine, part_2: bool) -> Option<isize> {
    let game_det = machine.a.x * machine.b.y - machine.a.y * machine.b.x;
    if game_det == 0 {
        panic!("Singular matrix");
    }
    let prod_tup = if part_2 {
        (
            (machine.b.y * (machine.prize_x + 10000000000000))
                - (machine.b.x * (machine.prize_y + 10000000000000)),
            (machine.a.x * (machine.prize_y + 10000000000000))
                - (machine.a.y * (machine.prize_x + 10000000000000)),
        )
    } else {
        (
            (machine.b.y * machine.prize_x) - (machine.b.x * machine.prize_y),
            (machine.a.x * machine.prize_y) - (machine.a.y * machine.prize_x),
        )
    };
    if prod_tup.0 % game_det != 0 || prod_tup.1 % game_det != 0 {
        return None;
    }
    return Some((3 * prod_tup.0 / game_det) + (1 * prod_tup.1 / game_det));
}

#[derive(Debug, PartialEq)]
struct Machine {
    a: Button,
    b: Button,
    prize_x: isize,
    prize_y: isize,
}

#[derive(Debug, PartialEq)]
struct Button {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let Ok(machines) = get_games(file_iter) else {
                panic!("unable to parse input")
            };
            let mut total = 0;
            for machine in machines {
                total += win_game_cost(machine).unwrap_or(0);
            }
            assert_eq!(total, 480);
        }
    }

    #[test]
    fn test_get_games() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let Ok(machines) = get_games(file_iter) else {
                panic!("unable to parse input")
            };
            assert_eq!(
                machines[0],
                Machine {
                    a: Button { x: 94, y: 34 },
                    b: Button { x: 22, y: 67 },
                    prize_x: 8400,
                    prize_y: 5400
                }
            );
        }
    }
}
