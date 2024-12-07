use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_7.txt") {
        let mut total = 0;
        let mut total2 = 0;
        for line in file_iter {
            let text = line.unwrap();
            total += get_correct_test_values_total(text.clone());
            total2 += test_values_include_concat(text);
        }
        println!("Part 1: {}", total);
        println!("Part 2: {}", total2);
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

fn test_values_include_concat(test_eq: String) -> u64 {
    let equation = process(test_eq);
    let num_factors = equation.1.len();
    let mut running_total;
    'keep_trying: for i in
        0..3usize.pow(<usize as TryInto<u32>>::try_into(num_factors).unwrap() - 1)
    {
        running_total = equation.1[0];
        for j in 1..num_factors {
            let op_selector = (i
                .checked_div(3usize.pow(<usize as TryInto<u32>>::try_into(j).unwrap() - 1)))
            .unwrap();
            match op_selector % 3 {
                0 => running_total = running_total + equation.1[j],
                1 => running_total = running_total * equation.1[j],
                2 => {
                    running_total = concat_nums(running_total, equation.1[j]);
                }
                _ => panic!("How"),
            }
        }
        if running_total > equation.0 {
            continue 'keep_trying;
        }
        if running_total == equation.0 {
            return equation.0;
        }
    }
    return 0;
}

fn concat_nums(a: u64, b: u64) -> u64 {
    let c = a * 10_u64.pow(b.ilog10() + 1) + b;
    return c;
}

fn get_correct_test_values_total(test_eq: String) -> u64 {
    let equation = process(test_eq);
    let num_factors = equation.1.len();
    let mut running_total;
    'keep_trying: for i in 0..2_u64.pow(<usize as TryInto<u32>>::try_into(num_factors).unwrap() - 1)
    {
        running_total = equation.1[0];
        for j in 0..num_factors - 1 {
            if (i >> j) & 0x00000001 == 1 {
                running_total = running_total * equation.1[j + 1];
            } else {
                running_total = running_total + equation.1[j + 1];
            }
            if running_total > equation.0 {
                continue 'keep_trying;
            }
        }
        if running_total == equation.0 {
            return equation.0;
        }
    }
    return 0;
}

fn process(line: String) -> (u64, Vec<u64>) {
    let split_line: Vec<&str> = line.split(" ").collect();
    let target = split_line[0][0..split_line[0].len() - 1]
        .parse::<u64>()
        .unwrap();
    let factors = split_line[1..split_line.len()]
        .iter()
        .map(|fac| {
            fac.parse::<u64>()
                .expect(&format!("Tried to unwrap {:?}", fac))
        })
        .collect();
    return (target, factors);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let mut total = 0;
            for line in file_iter {
                total += get_correct_test_values_total(line.unwrap());
            }
            assert_eq!(total, 3749);
        }
    }

    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let mut total: Vec<u64> = Vec::new();
            for line in file_iter {
                total.push(test_values_include_concat(line.unwrap()));
            }
            assert_eq!(total, vec![190, 3267, 156, 7290, 192, 292]);
        }
    }

    #[test]
    fn test_line_process() {
        let test_string = String::from("3267: 81 40 27");
        assert_eq!(process(test_string), (3267, vec![81, 40, 27]));
    }

    #[test]
    fn test_concat() {
        let a = 345;
        let b = 57;
        assert_eq!(concat_nums(a, b), 34557);
    }
}
