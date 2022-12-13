use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let size = input
        .lines()
        .map(|lines| {
            lines
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .filter(|pairs| {
            (pairs[0][0] <= pairs[1][0] && pairs[0][1] >= pairs[1][1])
                || (pairs[0][0] >= pairs[1][0] && pairs[0][1] <= pairs[1][1])
        })
        .count();

    println!("{}", size);
}
