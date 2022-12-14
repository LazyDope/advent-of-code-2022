use std::{collections::HashSet, fs};

fn main() {
    let num: u32 = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let len = line.len();
            let (left, right) = line.split_at(len / 2);
            let l_col: HashSet<char> = left.chars().collect();
            let r_col: HashSet<char> = right.chars().collect();
            let letter = l_col.intersection(&r_col).next().unwrap();
            match letter {
                'a'..='z' => u32::from(*letter) - 96,
                'A'..='Z' => u32::from(*letter) - 38,
                _ => 0,
            }
        })
        .sum();

    println!("{}", num);
}
