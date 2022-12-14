use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut lines = input.lines();

    let mut num = 0;
    loop {
        let mut three_lines = (&mut lines).take(3);
        let l_1 = three_lines.next();
        let l_2 = three_lines.next();
        let l_3 = three_lines.next();
        if l_1.is_none() || l_2.is_none() || l_3.is_none() {
            break;
        }
        let l_1: HashSet<char> = l_1.unwrap().chars().collect();
        let l_2: HashSet<char> = l_2.unwrap().chars().collect();
        let l_3: HashSet<char> = l_3.unwrap().chars().collect();
        let letter = l_1
            .intersection(&l_2)
            .filter(|x| l_3.contains(x))
            .next()
            .unwrap();
        num += match letter {
            'a'..='z' => u32::from(*letter) - 96,
            'A'..='Z' => u32::from(*letter) - 38,
            _ => 0,
        };
    }

    println!("{}", num);
}
