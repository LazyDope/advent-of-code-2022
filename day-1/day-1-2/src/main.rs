use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut max_vec: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();

    max_vec.sort();
    let len = max_vec.len();

    let max: u32 = max_vec[len - 3..].iter().sum();

    println!("{:?}", max);
}
