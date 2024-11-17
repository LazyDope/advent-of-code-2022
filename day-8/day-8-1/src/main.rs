use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let size: [usize; 2] = [trees.len(), trees.get(0).unwrap_or(&vec![]).len()];
    let mut visible = 2 * size[0] + 2 * size[1] - 4;

    for (i, row) in trees.iter().enumerate().take(size[0] - 1).skip(1) {
        for (j, height) in row.iter().enumerate().take(size[1] - 1).skip(1) {
            if row[0..j].iter().any(|t_h| t_h >= height)
                && row[j + 1..size[1]].iter().any(|t_h| t_h >= height)
                && trees[0..i].iter().any(|t_row| t_row[j] >= *height)
                && trees[i + 1..size[0]]
                    .iter()
                    .any(|t_row| t_row[j] >= *height)
            {
                continue;
            }
            visible += 1;
        }
    }

    println!("Result: {}", visible);
}
