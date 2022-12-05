use std::str::FromStr;

use anyhow::Result;

struct CratePile {
    columns: Vec<Vec<char>>,
}

struct Moves {
    crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for CratePile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut idx = 3;
            let mut col_num = 1;
            while idx <= line.len() {
                if columns.len() < col_num {
                    let col = Vec::new();
                    columns.push(col);
                }
                let current_col = &mut columns[col_num - 1];
                if line.get(idx - 3..idx).unwrap().starts_with("[") {
                    current_col.push(line.chars().nth(idx - 2).expect("aoc lied to me"));
                }
                idx += 4;
                col_num += 1;
            }
        }
        return Ok(CratePile { columns });
    }
}

impl CratePile {
    fn print_piles(&self) {
        println!("{:?}", self.columns);
    }

    fn print_tops(&self) {
        let mut ans = String::from("");
        for column in &self.columns {
            ans.push(column.last().unwrap().clone());
        }
        println!("{:?}", ans);
    }

    fn move_crate(&mut self, crate_move: Moves) {
        let mut crate_moves = crate_move.crates;
        while crate_moves > 0 {
            let col_to_move_from = self.columns.get_mut(crate_move.from - 1).unwrap();
            let popped = col_to_move_from.pop().unwrap();
            let cols_to_move_to = self.columns.get_mut(crate_move.to - 1).unwrap();
            cols_to_move_to.push(popped);
            crate_moves -= 1;
        }
    }

    fn move_crate_9001(&mut self, crate_move: Moves) {
        let col_to_move_from = &mut self.columns[crate_move.from - 1];
        let mut crate_moves = crate_move.crates;
        let mut crates_to_move = Vec::new();
        while crate_moves > 0 {
            let crate_to_move = col_to_move_from.pop().unwrap();
            crates_to_move.push(crate_to_move);
            crate_moves -= 1;
        }
        let cols_to_move_to = &mut self.columns[crate_move.to - 1];
        crates_to_move.reverse();
        cols_to_move_to.append(&mut crates_to_move);
    }

    fn apply_moves(&mut self, moves: Vec<Moves>) {
        for crate_move in moves {
            self.move_crate(crate_move);
        }
    }

    fn apply_moves_9001(&mut self, moves: Vec<Moves>) {
        for crate_move in moves {
            self.move_crate_9001(crate_move);
        }
    }
}

impl FromStr for Moves {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_dets: Vec<usize> = s
            .split(' ')
            .filter_map(|word| {
                if word.parse::<usize>().is_ok() {
                    Some(word.parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .collect();
        return Ok(Moves {
            crates: move_dets[0],
            from: move_dets[1],
            to: move_dets[2],
        });
    }
}

impl Moves {
    fn print_move(&self) {
        println!("{:?}", [self.crates, self.from, self.to]);
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input5.prod")?;

    let parsed_one = input.split_once("\n\n").unwrap();
    let mut crates = parsed_one.0.parse::<CratePile>()?;
    let moves_one: Vec<Moves> = parsed_one
        .1
        .lines()
        .map(|line| line.parse::<Moves>().unwrap())
        .collect();
    crates.print_piles();
    for move_one in &moves_one {
        move_one.print_move();
    }
    crates.apply_moves(moves_one);
    crates.print_piles();
    crates.print_tops();

    // println!("Part one: {:?}", parsed_one);

    return Ok(());
}
