use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::num::ParseIntError;
use std::path::Path;
use std::time::Instant;
extern crate bmp;
use bmp::{px, Image, Pixel};

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_14.txt") {
        let Ok(mut robot_vector) = get_initial_conditions(file_iter, 101, 103) else {
            panic!("Unable to parse input")
        };
        println!("robots: {}", robot_vector.len());
        for i in 0..(101 * 103) {
            let mut quadrants = (0, 0, 0, 0);
            for bot in &mut robot_vector {
                bot.mult(1);
                match bot.determine_quadrant() {
                    Some(Quadrant::UpperLeft) => quadrants.0 += 1,
                    Some(Quadrant::UpperRight) => quadrants.1 += 1,
                    Some(Quadrant::LowerLeft) => quadrants.2 += 1,
                    Some(Quadrant::LowerRight) => quadrants.3 += 1,
                    None => {}
                }
            }

            make_bmp(&robot_vector, 101, 103, i);
        }
        //println!("Total: {}", quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3);
    }
    println!("Finished in {:?}", now.elapsed());
}

fn get_initial_conditions(
    i_file: io::Lines<io::BufReader<File>>,
    x_limit: isize,
    y_limit: isize,
) -> Result<Vec<Robot>, ParseIntError> {
    let mut robot_vec: Vec<Robot> = Vec::new();
    let rob_regex: Regex =
        Regex::new(r"p\=([0-9]+)\,([0-9]+) v\=([-]*[0-9]+)\,([-]*[0-9]+)").unwrap();

    let all_lines: String = i_file.map(|line| line.unwrap()).collect();
    for (_, [x_pos, y_pos, x_vel, y_vel]) in
        rob_regex.captures_iter(&all_lines).map(|c| c.extract())
    {
        robot_vec.push(Robot {
            p_x: x_pos.parse::<isize>()?,
            p_y: y_pos.parse::<isize>()?,
            v_x: x_vel.parse::<isize>()?,
            v_y: y_vel.parse::<isize>()?,
            x_lim: x_limit,
            y_lim: y_limit,
        });
    }
    return Ok(robot_vec);
}

#[allow(dead_code)]
fn print_bots(bot_vec: &Vec<Robot>, x_lim: usize, y_lim: usize) {
    let mut field: Vec<Vec<usize>> = iter::repeat(iter::repeat(0).take(x_lim).collect())
        .take(y_lim)
        .collect();
    for bot in bot_vec {
        field[TryInto::<usize>::try_into(bot.p_y).unwrap()]
            [TryInto::<usize>::try_into(bot.p_x).unwrap()] += 1;
    }
    for row in field {
        let row_str: String = row
            .iter()
            .map(|c| {
                if *c > 0 {
                    c.to_string()
                } else {
                    " ".to_string()
                }
            })
            .collect();
        println!("{}", row_str);
    }
}

fn make_bmp(robot_vector: &Vec<Robot>, x_lim: usize, y_lim: usize, iter_number: usize) {
    let mut img = Image::new(x_lim.try_into().unwrap(), y_lim.try_into().unwrap());
    for bot in robot_vector {
        img.set_pixel(
            TryInto::<u32>::try_into(bot.p_x).unwrap(),
            TryInto::<u32>::try_into(bot.p_y).unwrap(),
            px!(255, 255, 255),
        );
    }
    let save_title = iter_number.to_string();
    let _ = img.save(&save_title);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Robot {
    p_x: isize,
    p_y: isize,
    v_x: isize,
    v_y: isize,
    x_lim: isize,
    y_lim: isize,
}

impl Robot {
    fn mult(&mut self, dist: isize) {
        let mut tmp_x = self.p_x + self.v_x * dist;
        while tmp_x < 0 {
            tmp_x += self.x_lim;
        }
        self.p_x = tmp_x % self.x_lim;
        let mut tmp_y = self.p_y + self.v_y * dist;
        while tmp_y < 0 {
            tmp_y += self.y_lim;
        }
        self.p_y = tmp_y % self.y_lim;
    }
    fn determine_quadrant(&self) -> Option<Quadrant> {
        match (
            (self.p_x * 2).cmp(&(self.x_lim - 1)),
            (self.p_y * 2).cmp(&(self.y_lim - 1)),
        ) {
            (Ordering::Less, Ordering::Less) => return Some(Quadrant::UpperLeft),
            (Ordering::Less, Ordering::Greater) => return Some(Quadrant::LowerLeft),
            (Ordering::Greater, Ordering::Less) => return Some(Quadrant::UpperRight),
            (Ordering::Greater, Ordering::Greater) => return Some(Quadrant::LowerRight),
            (_, _) => {
                return None;
            }
        }
    }
}

enum Quadrant {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let Ok(mut robot_vector) = get_initial_conditions(file_iter, 11, 7) else {
                panic!("Unable to parse input")
            };
            println!("robots: {}", robot_vector.len());
            print_bots(&robot_vector, 11, 7);
            let mut quadrants = (0, 0, 0, 0);
            for bot in &mut robot_vector {
                bot.mult(100);
                println!("bot position: {}, {}", bot.p_x, bot.p_y);
                match bot.determine_quadrant() {
                    Some(Quadrant::UpperLeft) => quadrants.0 += 1,
                    Some(Quadrant::UpperRight) => quadrants.1 += 1,
                    Some(Quadrant::LowerLeft) => quadrants.2 += 1,
                    Some(Quadrant::LowerRight) => quadrants.3 += 1,
                    None => {}
                }
            }
            assert_eq!(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3, 12);
        }
    }
}
