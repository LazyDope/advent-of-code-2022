use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (cargo, instructions) = input.split_once("\n\n").unwrap();
    let mut cargo: Cargo = cargo.into();
    instructions.lines().for_each(|instruction| {
        cargo.apply(instruction.into());
    });

    cargo.output();
}

struct Cargo {
    cargo: Vec<Vec<char>>,
}

impl Cargo {
    fn apply(&mut self, instruction: Instruction) {
        for _ in 0..instruction.count {
            let moving = self.cargo[instruction.start].pop();
            match moving {
                Some(x) => {
                    self.cargo[instruction.end].push(x);
                }
                None => break,
            }
        }
    }

    fn output(&self) {
        self.cargo
            .iter()
            .for_each(|stack| print!("{}", stack.last().unwrap_or(&' ')));
        println!();
    }
}

impl From<&str> for Cargo {
    fn from(string: &str) -> Cargo {
        let index_line = string.lines().last().unwrap();
        let size: usize = (index_line.len() + 1) / 4;
        let mut cargo: Vec<Vec<char>> = vec![vec![]; size];
        string.lines().rev().skip(1).for_each(|line| {
            let chars = line.chars().skip(1);
            for (index, part) in chars.step_by(4).enumerate() {
                match part {
                    letter @ 'A'..='Z' => {
                        cargo[index].push(letter);
                    }
                    ' ' => {}
                    _ => {
                        panic!("Incorrect data in input file!")
                    }
                };
            }
        });
        Cargo { cargo }
    }
}

struct Instruction {
    pub count: usize,
    pub start: usize,
    pub end: usize,
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Instruction {
        let (left, right) = instruction.split_once(" from ").unwrap();
        let count = left.strip_prefix("move ").unwrap().parse().unwrap();
        let (start, end) = right.split_once(" to ").unwrap();
        let start = start.parse::<usize>().unwrap() - 1;
        let end = end.parse::<usize>().unwrap() - 1;
        Instruction { count, start, end }
    }
}
