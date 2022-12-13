use std::fs;

fn main() {
    let score: u32 = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|battle| {
            let round: RPSRound = battle.into();
            let choice: RPSChoice;
            let mut round_score: u32 = match round {
                RPSRound::Win(x) => {
                    choice = x;
                    6
                }
                RPSRound::Draw(x) => {
                    choice = x;
                    3
                }
                RPSRound::Loss(x) => {
                    choice = x;
                    0
                }
            };
            match choice {
                RPSChoice::Rock => round_score += 1,
                RPSChoice::Paper => round_score += 2,
                RPSChoice::Scissors => round_score += 3,
            }
            round_score
        })
        .sum();

    println!("{}", score);
}

enum RPSRound {
    Win(RPSChoice),
    Draw(RPSChoice),
    Loss(RPSChoice),
}

enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RPSRound {
    fn from(other: &str) -> RPSRound {
        match other.split_once(' ') {
            Some(("A", "X")) => RPSRound::Draw(RPSChoice::Rock),
            Some(("B", "X")) => RPSRound::Loss(RPSChoice::Rock),
            Some(("C", "X")) => RPSRound::Win(RPSChoice::Rock),
            Some(("A", "Y")) => RPSRound::Win(RPSChoice::Paper),
            Some(("B", "Y")) => RPSRound::Draw(RPSChoice::Paper),
            Some(("C", "Y")) => RPSRound::Loss(RPSChoice::Paper),
            Some(("A", "Z")) => RPSRound::Loss(RPSChoice::Scissors),
            Some(("B", "Z")) => RPSRound::Win(RPSChoice::Scissors),
            Some(("C", "Z")) => RPSRound::Draw(RPSChoice::Scissors),
            _ => panic!("Oops!"),
        }
    }
}
