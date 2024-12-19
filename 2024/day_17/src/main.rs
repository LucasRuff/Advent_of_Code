use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(file_iter) = read_lines("input_17.txt") {
        let (mut machine, instructions) = process_input(file_iter);
        let mut output: Vec<usize> = Vec::new();
        while machine.ip < instructions.len() - 1 {
            match machine.process_instruction(Instruction {
                opcode: get_opcode(instructions[machine.ip]),
                operand: instructions[machine.ip + 1],
            }) {
                Some(val) => output.push(val),
                None => continue,
            }
        }
        println!("Part 1: {:?} in {:?}", output, now.elapsed());
        let best_a = find_input(instructions, false);

        println!("Part 2: {:?}", best_a);
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

fn find_input(instructions: Vec<usize>, debug: bool) -> Vec<usize> {
    let mut goal: usize = 0;
    for i in &instructions {
        goal <<= 3;
        goal |= i;
    }
    if debug {
        println!("Goal is {goal:b}");
    }
    let mut frontier: Vec<(usize, usize)> = vec![(0, 0)]; //this is the candidate a value and the number of outputs it matches
    let mut working_vals: Vec<usize> = Vec::new();
    loop {
        match frontier.pop() {
            Some((curr_val, matches)) => {
                if matches == instructions.len() {
                    working_vals.push(curr_val);
                } else {
                    let next_output = instructions[instructions.len() - matches - 1];
                    if debug {
                        println!(
                            "looking for candidates for output # {matches}: {}",
                            next_output
                        );
                    }
                    for i in 0..8 {
                        let next_val = (curr_val << 3) | i;
                        let mut test_machine = Machine {
                            reg_a: next_val,
                            reg_b: 0,
                            reg_c: 0,
                            ip: 0,
                        };
                        let mut test_output: Vec<usize> = Vec::new();

                        while test_machine.ip < instructions.len() - 1 {
                            match test_machine.process_instruction(Instruction {
                                opcode: get_opcode(instructions[test_machine.ip]),
                                operand: instructions[test_machine.ip + 1],
                            }) {
                                Some(val) => test_output.push(val),
                                None => continue,
                            }
                        }
                        if test_output[0] == instructions[instructions.len() - matches - 1] {
                            frontier.push((next_val, matches + 1));
                        }
                    }
                }
            }
            None => return working_vals,
        }
    }
}

fn process_input(i_file: io::Lines<io::BufReader<File>>) -> (Machine, Vec<usize>) {
    let all_input: String = i_file.map(|line| line.unwrap()).collect();
    let re_number: Regex = Regex::new(r"[0-9]+").unwrap();
    let numbers: Vec<usize> = re_number
        .find_iter(&all_input)
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect();
    let mut instruction_vec = Vec::new();
    for i in 3..numbers.len() {
        instruction_vec.push(numbers[i]);
    }

    return (
        Machine {
            ip: 0,
            reg_a: numbers[0],
            reg_b: numbers[1],
            reg_c: numbers[2],
        },
        instruction_vec,
    );
}

fn get_combo_operand(operand: usize, machine: &Machine) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => machine.reg_a,
        5 => machine.reg_b,
        6 => machine.reg_c,
        _ => panic!("Invalid operand"),
    }
}

fn get_opcode(opcode: usize) -> Opcode {
    match opcode {
        0 => Opcode::Adv,
        1 => Opcode::Bxl,
        2 => Opcode::Bst,
        3 => Opcode::Jnz,
        4 => Opcode::Bxc,
        5 => Opcode::Out,
        6 => Opcode::Bdv,
        7 => Opcode::Cdv,
        _ => panic!("Unable to read opcode"),
    }
}

#[derive(Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

struct Instruction {
    opcode: Opcode,
    operand: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Machine {
    ip: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

impl Machine {
    fn process_instruction(&mut self, instr: Instruction) -> Option<usize> {
        match instr.opcode {
            Opcode::Adv => {
                let oper = get_combo_operand(instr.operand, &self);
                self.reg_a = self.reg_a / (2_usize.pow(oper.try_into().unwrap()));
                self.ip += 2;
                return None;
            }
            Opcode::Bxl => {
                self.reg_b = self.reg_b ^ instr.operand;
                self.ip += 2;
                return None;
            }
            Opcode::Bst => {
                self.reg_b = get_combo_operand(instr.operand, &self) % 8;
                self.ip += 2;
                return None;
            }
            Opcode::Jnz => {
                match self.reg_a {
                    0 => {
                        self.ip += 2;
                    }
                    _ => {
                        self.ip = instr.operand;
                    }
                }
                return None;
            }
            Opcode::Bxc => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.ip += 2;
                return None;
            }
            Opcode::Out => {
                self.ip += 2;
                return Some(get_combo_operand(instr.operand, &self) % 8);
            }
            Opcode::Bdv => {
                let oper = get_combo_operand(instr.operand, &self);
                self.reg_b = self.reg_a / (2_usize.pow(oper.try_into().unwrap()));
                self.ip += 2;
                return None;
            }
            Opcode::Cdv => {
                let oper = get_combo_operand(instr.operand, &self);
                self.reg_c = self.reg_a / (2_usize.pow(oper.try_into().unwrap()));
                self.ip += 2;
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overall_test_1() {
        if let Ok(file_iter) = read_lines("test_input.txt") {
            let (mut machine, instructions) = process_input(file_iter);
            assert_eq!(
                machine,
                Machine {
                    ip: 0,
                    reg_a: 729,
                    reg_b: 0,
                    reg_c: 0
                }
            );
            assert_eq!(instructions, vec![0, 1, 5, 4, 3, 0]);
            let mut output: Vec<usize> = Vec::new();
            while machine.ip < instructions.len() - 1 {
                match machine.process_instruction(Instruction {
                    opcode: get_opcode(instructions[machine.ip]),
                    operand: instructions[machine.ip + 1],
                }) {
                    Some(val) => output.push(val),
                    None => continue,
                }
            }
            assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
        }
    }
    #[test]
    fn overall_test_2() {
        if let Ok(file_iter) = read_lines("test_2.txt") {
            let (mut machine, instructions) = process_input(file_iter);

            let best_as = find_input(instructions.clone(), true);
            machine.reg_a = best_as[best_as.len() - 1];
            let mut output: Vec<usize> = Vec::new();
            while machine.ip < instructions.len() - 1 {
                match machine.process_instruction(Instruction {
                    opcode: get_opcode(instructions[machine.ip]),
                    operand: instructions[machine.ip + 1],
                }) {
                    Some(val) => output.push(val),
                    None => continue,
                }
            }
            assert_eq!(output, instructions);
        }
    }
}
