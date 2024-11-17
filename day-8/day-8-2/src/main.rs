use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let best_tree = trees
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &height)| {
                    row[0..j]
                        .iter()
                        .rev()
                        .take_while_inclusive(|&&t_h| t_h < height)
                        .count()
                        * row[j + 1..]
                            .iter()
                            .take_while_inclusive(|&&t_h| t_h < height)
                            .count()
                        * trees[0..i]
                            .iter()
                            .rev()
                            .map(|t_row| t_row[j])
                            .take_while_inclusive(|&t_h| t_h < height)
                            .count()
                        * trees[i + 1..]
                            .iter()
                            .map(|t_row| t_row[j])
                            .take_while_inclusive(|&t_h| t_h < height)
                            .count()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Result: {}", best_tree);
}
