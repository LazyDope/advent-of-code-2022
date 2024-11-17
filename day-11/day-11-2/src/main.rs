use std::{fmt::Debug, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut multi_divisor = 1;
    for monkey_def in input.split("\n\n") {
        let mut lines = monkey_def.lines().map(str::trim);
        let items: Vec<u64> = lines
            .find_map(|v| {
                Some(
                    v.split_once("Starting items: ")?
                        .1
                        .split(", ")
                        .map(|v| v.parse().unwrap())
                        .collect(),
                )
            })
            .unwrap();
        let operation: Operation = field_from_lines(&mut lines, "Operation: new = old ");
        let divisor: u64 = field_from_lines(&mut lines, "Test: divisible by ");
        multi_divisor *= divisor;
        let true_target: usize = field_from_lines(&mut lines, "If true: throw to monkey ");
        let false_target: usize = field_from_lines(&mut lines, "If false: throw to monkey ");
        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            true_target,
            false_target,
            monkey_business: 0,
        })
    }
    println!("{}", multi_divisor);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let (left_monkeys, right_monkeys) = monkeys.split_at_mut(i);
            let (curr_monkey, right_monkeys) = right_monkeys.split_first_mut().unwrap();
            curr_monkey.monkey_business(left_monkeys, right_monkeys, i, multi_divisor);
        }
    }

    let top_2: [usize; 2] = monkeys.iter().fold([0, 0], |mut acc, monkey| {
        if monkey.monkey_business > acc[0] {
            acc[0] = monkey.monkey_business
        }
        if acc[0] > acc[1] {
            acc.swap(0, 1)
        }
        acc
    });

    println!("{}", top_2.iter().product::<usize>())
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_target: usize,
    false_target: usize,
    monkey_business: usize,
}

enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

fn field_from_lines<'a, T>(lines: &mut impl Iterator<Item = &'a str>, field: &str) -> T
where
    T: FromStr<Err: Debug>,
{
    lines
        .find_map(|v| Some(v.split_once(field)?.1.parse().unwrap()))
        .unwrap()
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Add(other) => value + other,
            Operation::Mul(other) => value * other,
            Operation::Square => value * value,
        }
    }
}

impl Monkey {
    fn monkey_business(
        &mut self,
        left_monkeys: &mut [Monkey],
        right_monkeys: &mut [Monkey],
        i: usize,
        multi_divisor: u64,
    ) {
        self.monkey_business += self.items.len();
        for mut item in self.items.drain(..) {
            item = self.operation.apply(item) % multi_divisor;
            if item % self.divisor == 0 {
                if self.true_target < i {
                    &mut left_monkeys[self.true_target]
                } else {
                    &mut right_monkeys[self.true_target - i - 1]
                }
            } else if self.false_target < i {
                &mut left_monkeys[self.false_target]
            } else {
                &mut right_monkeys[self.false_target - i - 1]
            }
            .items
            .push(item)
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(" ").ok_or("Missing value")?;
        Ok(match l {
            "+" => Operation::Add(r.parse().map_err(|_| "Invalid value")?),
            "*" => match r {
                "old" => Operation::Square,
                x => Operation::Mul(x.parse().map_err(|_| "Invalid value")?),
            },
            _ => return Err("Invalid operation"),
        })
    }
}
