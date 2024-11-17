use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut cycle = 0;
    let mut x = 1;
    for line in input.lines() {
        cycle_checks(&mut cycle, x);
        #[allow(clippy::single_match)]
        match &line[..4] {
            "addx" => {
                cycle_checks(&mut cycle, x);
                let val: i32 = line[5..]
                    .parse()
                    .expect("Value for addx should always be a number");
                x += val;
            }
            _ => (),
        }
    }
}

fn cycle_checks(cycle: &mut i32, x: i32) {
    if *cycle != 0 && (*cycle % 40) == 0 {
        println!();
    }
    if ((x - 1)..=(x + 1)).contains(&(*cycle % 40)) {
        print!("#");
    } else {
        print!(".");
    }
    *cycle += 1;
}
