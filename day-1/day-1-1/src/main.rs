use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let max: u32 = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("{}", max);
}
