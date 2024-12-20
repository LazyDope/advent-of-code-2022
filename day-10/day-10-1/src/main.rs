use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for line in input.lines() {
        cycle_checks(&mut cycle, &mut sum, x);
        #[allow(clippy::single_match)]
        match &line[..4] {
            "addx" => {
                cycle_checks(&mut cycle, &mut sum, x);
                let val: i32 = line[5..]
                    .parse()
                    .expect("Value for addx should always be a number");
                x += val;
            }
            _ => (),
        }
    }
    println!("{sum}")
}

fn cycle_checks(cycle: &mut i32, sum: &mut i32, x: i32) {
    *cycle += 1;
    if (*cycle % 40) == 20 {
        *sum += x * *cycle;
    }
}
