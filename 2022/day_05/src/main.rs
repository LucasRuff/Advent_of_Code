use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let lines_iter = get_line_iter("input.txt").unwrap().lines();
    let mut elf_warehouse = Warehouse::new();

    for line in lines_iter {
        let unwrapped_line = line.unwrap();
        let next_action :Action = parse_action(unwrapped_line);
        elf_warehouse = elf_warehouse.multi_stack(next_action);
    }
    elf_warehouse.pretty_print();

}

fn get_line_iter(filename :&str) -> Option<std::io::BufReader<std::fs::File>> {
    let file = File::open(filename).ok()?;
    Some(BufReader::new(file))
}

fn parse_action(line :String) -> Action {
    let parsed_line :Vec<&str>= line.split(' ').collect();
    Action {
        number_of_crates: parsed_line[1].parse().unwrap(),
        starting_stack: parsed_line[3].parse().unwrap(),
        ending_stack: parsed_line[5].parse().unwrap()
    }
}

struct Action {
    pub number_of_crates: usize,
    pub starting_stack: usize,
    pub ending_stack: usize
}

struct Warehouse {
    pub stacks: Vec<Vec<char>>
}

impl Warehouse {
    pub fn new() -> Self {
        let mut stack_one :Vec<char> = vec!['Z','P','B','Q','M','D','N'];
        let mut stack_two :Vec<char> = vec!['V','H','D','M','Q','Z','L','C'];
        let mut stack_three :Vec<char> = vec!['G','Z','F','V','D','R','H','Q'];
        let mut stack_four :Vec<char> = vec!['N','F','D','G','H'];
        let mut stack_five :Vec<char> = vec!['Q','F','N'];
        let mut stack_six :Vec<char> = vec!['T','B','F','Z','V','D','Z','T','M','Q'];
        let mut stack_seven :Vec<char> = vec!['H','S','V','D','Z','T','M','Q'];
        let mut stack_eight :Vec<char> = vec!['Q','N','P','F','G','M'];
        let mut stack_nine = vec!['M','R','W','B'];
        stack_one.reverse();
        stack_two.reverse();
        stack_three.reverse();
        stack_four.reverse();
        stack_five.reverse();
        stack_six.reverse();
        stack_seven.reverse();
        stack_eight.reverse();
        stack_nine.reverse();
        let mut stacker = Vec::new();
        stacker.push(stack_one);
        stacker.push(stack_two);
        stacker.push(stack_three);
        stacker.push(stack_four);
        stacker.push(stack_five);
        stacker.push(stack_six);
        stacker.push(stack_seven);
        stacker.push(stack_eight);
        stacker.push(stack_nine);

        Self{
            stacks: stacker
        }
    }
    pub fn single_stack(mut self, action :Action) -> Self {
        let mut holding_area :char;
        for _ in 0..(action.number_of_crates) {
            holding_area = self.stacks[action.starting_stack-1].pop().unwrap();
            //let mut temp_stack = self.stacks[action.starting_stack-1].clone();
            //println!("Popping {holding_area} from stack {}: {:?}", action.starting_stack, temp_stack);
            //temp_stack = self.stacks[action.ending_stack-1].clone();
            //println!("Pushing {holding_area} to stack {}: {:?}", action.ending_stack, temp_stack);
            self.stacks[action.ending_stack-1].push(holding_area);
        }
        self
    }
    pub fn multi_stack(mut self, action :Action) -> Self {
        let mut holding_area :Vec<char> = Vec::new();
        for _ in 0..(action.number_of_crates) {
            holding_area.push(self.stacks[action.starting_stack-1].pop().unwrap());
        }
        for _ in 0..(action.number_of_crates) {
            self.stacks[action.ending_stack-1].push(holding_area.pop().unwrap());
        }
        self
    }
    pub fn pretty_print(self) {
        for i in 0..self.stacks.len() {
            println!("Stack {i}: {:?}", self.stacks[i]);
        }
    }
}