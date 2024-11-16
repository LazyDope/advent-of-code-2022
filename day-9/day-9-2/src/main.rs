use std::fs;

struct Grid {
    seen_tiles: Vec<[i32; 2]>,
    tail_pos: [i32; 2],
    rel_head_pos: [i32; 2],
}

impl Grid {
    pub fn up(&mut self) {
        self.rel_head_pos[1] += 1;
        self.update_tail();
    }

    pub fn down(&mut self) {
        self.rel_head_pos[1] -= 1;
        self.update_tail();
    }

    pub fn left(&mut self) {
        self.rel_head_pos[0] -= 1;
        self.update_tail();
    }

    pub fn right(&mut self) {
        self.rel_head_pos[0] += 1;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let old_tail_pos = self.tail_pos;

        if self.rel_head_pos[0].abs() == 2 {
            self.tail_pos[0] += self.rel_head_pos[0].signum();
            self.rel_head_pos[0] -= self.rel_head_pos[0].signum();
            if self.rel_head_pos[1] != 0 {
                self.tail_pos[1] += self.rel_head_pos[1];
                self.rel_head_pos[1] = 0;
            }
        } else if self.rel_head_pos[1].abs() == 2 {
            self.tail_pos[1] += self.rel_head_pos[1].signum();
            self.rel_head_pos[1] -= self.rel_head_pos[1].signum();
            if self.rel_head_pos[0] != 0 {
                self.tail_pos[0] += self.rel_head_pos[0];
                self.rel_head_pos[0] = 0;
            }
        }
        if old_tail_pos != self.tail_pos {
            let tail_pos = self.tail_pos;
            if !self.seen_tiles.contains(&tail_pos) {
                self.seen_tiles.push(tail_pos);
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            seen_tiles: vec![[0, 0]],
            tail_pos: [0, 0],
            rel_head_pos: [0, 0],
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut grid = Grid::default();

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
        }
    }
    println!("{}", grid.seen_tiles.len());
}
