use clearscreen::ClearScreen;
use std::{
    fmt::Debug,
    fs,
    io::{self, Write},
    iter, thread,
    time::Duration,
};

struct Grid {
    seen_tiles: Vec<Position>,
    head_tiles: Vec<Position>,
    tails: [Position; 9],
    head: Position,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Grid {
    pub fn up(&mut self) {
        let old_start = self.head;
        self.head.y -= 1;
        self.update_tail(old_start);
    }

    pub fn down(&mut self) {
        let old_start = self.head;
        self.head.y += 1;
        self.update_tail(old_start);
    }

    pub fn left(&mut self) {
        let old_start = self.head;
        self.head.x -= 1;
        self.update_tail(old_start);
    }

    pub fn right(&mut self) {
        let old_start = self.head;
        self.head.x += 1;
        self.update_tail(old_start);
    }

    fn update_tail(&mut self, old_start: Position) {
        let old_end = self.tails[8];
        self.tails[0].update_from(&self.head);
        let mut slice = self.tails.as_mut_slice();
        for _ in 1..slice.len() {
            let (first, second) = slice.split_at_mut(1);
            second[0].update_from(&first[0]);
            slice = second;
        }
        let new_end = self.tails[8];
        if old_end != new_end && !self.seen_tiles.contains(&new_end) {
            self.seen_tiles.push(new_end);
        }
        let new_start = self.head;
        if old_start != new_start && !self.head_tiles.contains(&new_start) {
            self.head_tiles.push(new_start);
        }
    }
}

impl Position {
    fn update_from(&mut self, head: &Position) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;
        if dx.abs() >= 2 {
            self.x += dx.signum();
            if dy != 0 {
                self.y += dy.signum();
            }
        } else if dy.abs() >= 2 {
            self.y += dy.signum();
            if dx != 0 {
                self.x += dx.signum();
            }
        }
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            seen_tiles: vec![Position::new(0, 0)],
            head_tiles: vec![Position::new(0, 0)],
            tails: [Position::new(0, 0); 9],
            head: Position::new(0, 0),
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = self
            .seen_tiles
            .iter()
            .chain(self.head_tiles.iter())
            .chain(self.tails.iter())
            .chain(iter::once(&self.head))
            .fold((0, 0, 0, 0), |mut acc, v| {
                if v.x > acc.1 {
                    acc.1 = v.x
                } else if v.x < acc.0 {
                    acc.0 = v.x
                }
                if v.y > acc.3 {
                    acc.3 = v.y
                } else if v.y < acc.2 {
                    acc.2 = v.y
                }
                acc
            });
        writeln!(f, "{}", "-".repeat((max_x - min_x + 1) as usize))?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let curr_pos = Position::new(x, y);
                if self.head == curr_pos {
                    write!(f, "H")?;
                } else if let Some(i) = self.tails.iter().position(|v| *v == curr_pos) {
                    write!(f, "{}", i + 1)?;
                } else if Position::new(0, 0) == curr_pos {
                    write!(f, "s")?;
                } else if self.seen_tiles.contains(&curr_pos) && self.head_tiles.contains(&curr_pos)
                {
                    write!(f, "X")?;
                } else if self.seen_tiles.contains(&curr_pos) {
                    write!(f, "#")?;
                } else if self.head_tiles.contains(&curr_pos) {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", "-".repeat((max_x - min_x + 1) as usize))
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut grid = Grid::default();

    // let clearscreen = ClearScreen::default();
    for line in input.lines() {
        let (dir, num) = line.split_once(' ').expect("invalid move statement");
        let num: usize = num.parse().expect("invalid number of moves");
        let move_fn = match dir {
            "U" => Grid::up,
            "D" => Grid::down,
            "L" => Grid::left,
            "R" => Grid::right,
            _ => panic!("invalid direction"),
        };

        for _ in 0..num {
            move_fn(&mut grid);
            // let mut buf: Vec<u8> = vec![];
            // let _ = clearscreen.clear_to(&mut buf);
            // let _ = writeln!(&mut buf, "current: {}", line);
            // let _ = write!(&mut buf, "{:?}", grid);
            // let _ = io::stdout().write_all(&buf);
        }
    }
    println!("{}", grid.seen_tiles.len());
}
