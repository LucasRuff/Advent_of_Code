use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_15.txt") {
        let (mut arena, directions) = process_input(file_iter);
        //println!("{}", arena);
        move_robot(&mut arena, directions);
        println!("Total: {}", get_score(&arena));
    }
    println!("Finished part 1 in {:?}", now.elapsed());
    if let Ok(file_iter) = read_lines("input_15.txt") {
        let (mut arena_2, directions) = process_input_2(file_iter);
        //println!("{}", arena_2);
        move_robot_2(&mut arena_2, directions);
        println!("Total 2: {}", get_score(&arena_2));
    }
    println!("Finished part 2 in {:?}", now.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input_2(i_file: io::Lines<io::BufReader<File>>) -> (Arena, Vec<Direction>) {
    let mut is_map = true;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<Direction> = Vec::new();
    let mut robot_x = 0;
    let mut robot_y = 0;
    for (i, line) in i_file.enumerate() {
        let text = line.unwrap();
        if text.len() == 0 {
            is_map = false;
            continue;
        }
        if is_map {
            let mut build_row: Vec<char> = Vec::new();
            for (j, cha) in text.chars().enumerate() {
                match cha {
                    'O' => {
                        build_row.push('[');
                        build_row.push(']');
                    }
                    '#' => {
                        build_row.push('#');
                        build_row.push('#');
                    }
                    '@' => {
                        robot_x = j * 2;
                        robot_y = i;

                        build_row.push('.');
                        build_row.push('.');
                    }
                    _ => {
                        build_row.push('.');
                        build_row.push('.');
                    }
                }
            }
            map.push(build_row);
        } else {
            for cha in text.chars() {
                directions.push(match cha {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => {
                        panic!("unable to read direction")
                    }
                });
            }
        }
    }
    return (
        Arena {
            rob_x: robot_x,
            rob_y: robot_y,
            map: map,
        },
        directions,
    );
}

fn move_robot_2(mut arena: &mut Arena, dirs: Vec<Direction>) {
    for dir in dirs {
        let curr_x = arena.rob_x;
        let curr_y = arena.rob_y;
        match dir {
            Direction::Up => match arena.map[curr_y - 1][curr_x] {
                '#' => continue,
                '.' => {
                    arena.rob_y -= 1;
                }
                ']' => {
                    if move_up_2(&mut arena, curr_x, curr_y - 1) {
                        arena.map[curr_y - 2][curr_x] = ']';
                        arena.map[curr_y - 2][curr_x - 1] = '[';
                        arena.map[curr_y - 1][curr_x] = '.';
                        arena.map[curr_y - 1][curr_x - 1] = '.';
                        arena.rob_y -= 1;
                    }
                }
                '[' => {
                    if move_up_2(&mut arena, curr_x + 1, curr_y - 1) {
                        arena.map[curr_y - 2][curr_x] = '[';
                        arena.map[curr_y - 2][curr_x + 1] = ']';
                        arena.map[curr_y - 1][curr_x] = '.';
                        arena.map[curr_y - 1][curr_x + 1] = '.';
                        arena.rob_y -= 1;
                    }
                }
                _ => panic!(),
            },
            Direction::Down => match arena.map[curr_y + 1][curr_x] {
                '#' => continue,
                '.' => {
                    arena.rob_y += 1;
                }
                ']' => {
                    if move_down_2(&mut arena, curr_x, curr_y + 1) {
                        arena.map[curr_y + 2][curr_x] = ']';
                        arena.map[curr_y + 2][curr_x - 1] = '[';
                        arena.map[curr_y + 1][curr_x] = '.';
                        arena.map[curr_y + 1][curr_x - 1] = '.';
                        arena.rob_y += 1;
                    }
                }
                '[' => {
                    if move_down_2(&mut arena, curr_x + 1, curr_y + 1) {
                        arena.map[curr_y + 2][curr_x] = '[';
                        arena.map[curr_y + 2][curr_x + 1] = ']';
                        arena.map[curr_y + 1][curr_x] = '.';
                        arena.map[curr_y + 1][curr_x + 1] = '.';
                        arena.rob_y += 1;
                    }
                }
                _ => panic!(),
            },
            Direction::Left => match arena.map[curr_y][curr_x - 1] {
                '#' => continue,
                '.' => {
                    arena.rob_x -= 1;
                }
                ']' => {
                    if move_left_2(&mut arena, curr_x - 1, curr_y) {
                        arena.map[curr_y][curr_x - 1] = '.';
                        arena.rob_x -= 1;
                    }
                }
                _ => {
                    println!("{}", arena);
                    panic!(
                        " moving left into char {} at {}, {}",
                        arena.map[curr_y][curr_x - 1],
                        curr_y,
                        curr_x
                    );
                }
            },
            Direction::Right => match arena.map[curr_y][curr_x + 1] {
                '#' => continue,
                '.' => {
                    arena.rob_x += 1;
                }
                '[' => {
                    if move_right_2(&mut arena, curr_x + 1, curr_y) {
                        arena.map[curr_y][curr_x + 1] = '.';
                        arena.rob_x += 1;
                    }
                }
                _ => {
                    println!("{}", arena);
                    panic!(
                        " moving right into char {} at {}, {}",
                        arena.map[curr_y][curr_x + 1],
                        curr_y,
                        curr_x
                    );
                }
            },
        }
    }
}

fn move_down_2(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match (arena.map[loc_y + 1][loc_x - 1], arena.map[loc_y + 1][loc_x]) {
        (_, '#') | ('#', _) => return false,
        ('.', '.') => {
            arena.map[loc_y + 1][loc_x] = ']';
            arena.map[loc_y + 1][loc_x - 1] = '[';
            return true;
        }
        ('[', ']') => {
            if can_move_down(arena, loc_x, loc_y + 1) {
                _ = move_down_2(arena, loc_x, loc_y + 1);
                return true;
            }
            return false;
        }
        (']', '.') => {
            if can_move_down(arena, loc_x - 1, loc_y + 1) {
                _ = move_down_2(arena, loc_x - 1, loc_y + 1);
                arena.map[loc_y + 1][loc_x - 1] = '[';
                arena.map[loc_y + 1][loc_x] = ']';
                arena.map[loc_y + 1][loc_x - 2] = '.';
                return true;
            }
            return false;
        }
        ('.', '[') => {
            if can_move_down(arena, loc_x + 1, loc_y + 1) {
                _ = move_down_2(arena, loc_x + 1, loc_y + 1);
                arena.map[loc_y + 1][loc_x - 1] = '[';
                arena.map[loc_y + 1][loc_x] = ']';
                arena.map[loc_y + 1][loc_x + 1] = '.';
                return true;
            }
            return false;
        }
        (']', '[') => {
            if can_move_down(arena, loc_x + 1, loc_y + 1)
                && can_move_down(arena, loc_x - 1, loc_y + 1)
            {
                _ = move_down_2(arena, loc_x + 1, loc_y + 1);
                _ = move_down_2(arena, loc_x - 1, loc_y + 1);
                arena.map[loc_y + 1][loc_x - 1] = '[';
                arena.map[loc_y + 1][loc_x - 2] = '.';
                arena.map[loc_y + 1][loc_x] = ']';
                arena.map[loc_y + 1][loc_x + 1] = '.';
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn can_move_down(arena: &Arena, loc_x: usize, loc_y: usize) -> bool {
    match (arena.map[loc_y + 1][loc_x - 1], arena.map[loc_y + 1][loc_x]) {
        (_, '#') | ('#', _) => return false,
        ('.', '.') => {
            return true;
        }
        ('[', ']') => {
            if can_move_down(arena, loc_x, loc_y + 1) {
                return true;
            }
            return false;
        }
        (']', '.') => {
            if can_move_down(arena, loc_x - 1, loc_y + 1) {
                return true;
            }
            return false;
        }
        ('.', '[') => {
            if can_move_down(arena, loc_x + 1, loc_y + 1) {
                return true;
            }
            return false;
        }
        (']', '[') => {
            if can_move_down(arena, loc_x + 1, loc_y + 1)
                && can_move_down(arena, loc_x - 1, loc_y + 1)
            {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_up_2(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match (arena.map[loc_y - 1][loc_x - 1], arena.map[loc_y - 1][loc_x]) {
        (_, '#') | ('#', _) => return false,
        ('.', '.') => {
            arena.map[loc_y - 1][loc_x] = ']';
            arena.map[loc_y - 1][loc_x - 1] = '[';
            return true;
        }
        ('[', ']') => {
            if can_move_up(arena, loc_x, loc_y - 1) {
                _ = move_up_2(arena, loc_x, loc_y - 1);
                return true;
            }
            return false;
        }
        (']', '.') => {
            if can_move_up(arena, loc_x - 1, loc_y - 1) {
                _ = move_up_2(arena, loc_x - 1, loc_y - 1);
                arena.map[loc_y - 1][loc_x - 1] = '[';
                arena.map[loc_y - 1][loc_x - 2] = '.';
                arena.map[loc_y - 1][loc_x] = ']';
                return true;
            }
            return false;
        }
        ('.', '[') => {
            if can_move_up(arena, loc_x + 1, loc_y - 1) {
                _ = move_up_2(arena, loc_x + 1, loc_y - 1);
                arena.map[loc_y - 1][loc_x - 1] = '[';
                arena.map[loc_y - 1][loc_x + 1] = '.';
                arena.map[loc_y - 1][loc_x] = ']';
                return true;
            }
            return false;
        }
        (']', '[') => {
            if can_move_up(arena, loc_x + 1, loc_y - 1) && can_move_up(arena, loc_x - 1, loc_y - 1)
            {
                _ = move_up_2(arena, loc_x + 1, loc_y - 1);
                _ = move_up_2(arena, loc_x - 1, loc_y - 1);
                arena.map[loc_y - 1][loc_x - 1] = '[';
                arena.map[loc_y - 1][loc_x - 2] = '.';
                arena.map[loc_y - 1][loc_x] = ']';
                arena.map[loc_y - 1][loc_x + 1] = '.';
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn can_move_up(arena: &Arena, loc_x: usize, loc_y: usize) -> bool {
    match (arena.map[loc_y - 1][loc_x - 1], arena.map[loc_y - 1][loc_x]) {
        (_, '#') | ('#', _) => return false,
        ('.', '.') => {
            return true;
        }
        ('[', ']') => {
            if can_move_up(arena, loc_x, loc_y - 1) {
                return true;
            }
            return false;
        }
        (']', '.') => {
            if can_move_up(arena, loc_x - 1, loc_y - 1) {
                return true;
            }
            return false;
        }
        ('.', '[') => {
            if can_move_up(arena, loc_x + 1, loc_y - 1) {
                return true;
            }
            return false;
        }
        (']', '[') => {
            if can_move_up(arena, loc_x + 1, loc_y - 1) && can_move_up(arena, loc_x - 1, loc_y - 1)
            {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_left_2(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y][loc_x - 2] {
        '#' => return false,
        '.' => {
            arena.map[loc_y][loc_x - 2] = '[';
            arena.map[loc_y][loc_x - 1] = ']';
            return true;
        }
        ']' => {
            if move_left_2(arena, loc_x - 2, loc_y) {
                arena.map[loc_y][loc_x - 2] = '[';
                arena.map[loc_y][loc_x - 1] = ']';
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_right_2(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y][loc_x + 2] {
        '#' => return false,
        '.' => {
            arena.map[loc_y][loc_x + 2] = ']';
            arena.map[loc_y][loc_x + 1] = '[';
            return true;
        }
        '[' => {
            if move_right_2(arena, loc_x + 2, loc_y) {
                arena.map[loc_y][loc_x + 2] = ']';
                arena.map[loc_y][loc_x + 1] = '[';
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn process_input(i_file: io::Lines<io::BufReader<File>>) -> (Arena, Vec<Direction>) {
    let mut is_map = true;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut directions: Vec<Direction> = Vec::new();
    let mut robot_x = 0;
    let mut robot_y = 0;
    for (i, line) in i_file.enumerate() {
        let text = line.unwrap();
        if text.len() == 0 {
            is_map = false;
            continue;
        }
        if is_map {
            let mut build_row: Vec<char> = Vec::new();
            for (j, cha) in text.chars().enumerate() {
                if cha == '@' {
                    robot_x = j;
                    robot_y = i;
                    build_row.push('.');
                    continue;
                }
                build_row.push(cha);
            }
            map.push(build_row);
        } else {
            for cha in text.chars() {
                directions.push(match cha {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => {
                        panic!("unable to read direction")
                    }
                });
            }
        }
    }
    return (
        Arena {
            rob_x: robot_x,
            rob_y: robot_y,
            map: map,
        },
        directions,
    );
}

fn move_robot(mut arena: &mut Arena, dirs: Vec<Direction>) {
    for dir in dirs {
        let curr_x = arena.rob_x;
        let curr_y = arena.rob_y;
        match dir {
            Direction::Up => match arena.map[curr_y - 1][curr_x] {
                '#' => continue,
                '.' => arena.rob_y -= 1,
                'O' => {
                    if move_up(&mut arena, curr_x, curr_y - 1) {
                        arena.map[curr_y - 1][curr_x] = '.';
                        arena.rob_y -= 1;
                    }
                }
                _ => panic!(),
            },
            Direction::Down => match arena.map[curr_y + 1][curr_x] {
                '#' => continue,
                '.' => arena.rob_y += 1,
                'O' => {
                    if move_down(&mut arena, curr_x, curr_y + 1) {
                        arena.map[curr_y + 1][curr_x] = '.';
                        arena.rob_y += 1;
                    }
                }
                _ => panic!(),
            },
            Direction::Left => match arena.map[curr_y][curr_x - 1] {
                '#' => continue,
                '.' => arena.rob_x -= 1,
                'O' => {
                    if move_left(&mut arena, curr_x - 1, curr_y) {
                        arena.map[curr_y][curr_x - 1] = '.';
                        arena.rob_x -= 1;
                    }
                }
                _ => panic!(),
            },
            Direction::Right => match arena.map[curr_y][curr_x + 1] {
                '#' => continue,
                '.' => arena.rob_x += 1,
                'O' => {
                    if move_right(&mut arena, curr_x + 1, curr_y) {
                        arena.map[curr_y][curr_x + 1] = '.';
                        arena.rob_x += 1;
                    }
                }
                _ => panic!(),
            },
        }
    }
}

fn move_right(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y][loc_x + 1] {
        '#' => return false,
        '.' => {
            arena.map[loc_y][loc_x + 1] = 'O';
            return true;
        }
        'O' => {
            if move_right(arena, loc_x + 1, loc_y) {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_left(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y][loc_x - 1] {
        '#' => return false,
        '.' => {
            arena.map[loc_y][loc_x - 1] = 'O';
            return true;
        }
        'O' => {
            if move_left(arena, loc_x - 1, loc_y) {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_down(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y + 1][loc_x] {
        '#' => return false,
        '.' => {
            arena.map[loc_y + 1][loc_x] = 'O';
            return true;
        }
        'O' => {
            if move_down(arena, loc_x, loc_y + 1) {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn move_up(arena: &mut Arena, loc_x: usize, loc_y: usize) -> bool {
    match arena.map[loc_y - 1][loc_x] {
        '#' => return false,
        '.' => {
            arena.map[loc_y - 1][loc_x] = 'O';
            return true;
        }
        'O' => {
            if move_up(arena, loc_x, loc_y - 1) {
                return true;
            }
            return false;
        }
        _ => panic!("unable to read map character at {}, {}", loc_y, loc_x),
    }
}

fn get_score(arena: &Arena) -> usize {
    let mut total = 0;
    for (i, row) in arena.map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'O' || *col == '[' {
                total += 100 * i + j;
            }
        }
    }
    return total;
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut build_str = String::new();

        for row in &self.map {
            let mut tmp_str = String::new();
            for cha in row {
                tmp_str.push(*cha);
            }
            tmp_str.push('\n');
            build_str.push_str(&tmp_str);
        }
        write!(f, "{}", build_str)
    }
}

struct Arena {
    rob_x: usize,
    rob_y: usize,
    map: Vec<Vec<char>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (mut arena, mut directions) = process_input(file_iter);
            println!("Directions: {:?}", directions);
            move_robot(&mut arena, directions);
            println!("{:?}", arena.map);
            assert_eq!(get_score(&arena), 10092);
        }
    }

    #[test]
    fn test_process_input() {
        if let Ok(file_iter) = read_lines("easy_input.txt") {
            let (map, directions) = process_input(file_iter);
            let dirs: Vec<Direction> = vec![
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
            ];
            assert_eq!(directions, dirs);
            assert_eq!((map.rob_x, map.rob_y), (2, 2));
        }
    }
}
