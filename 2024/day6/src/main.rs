use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input6.txt").unwrap();
    let mut board = Board::from_str(&input);
    board.walk();
}

struct Board {
    floor: Vec<Vec<char>>,
    start: [usize; 2],
    rows: usize,
    cols: usize,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let mut start = [0, 0];
        let floor = input
            .lines()
            .enumerate()
            .fold(vec![], |mut acc, (i, line)| {
                let row: Vec<char> = line
                    .chars()
                    .enumerate()
                    .map(|(j, char)| {
                        if char == '^' {
                            start = [i, j];
                            return 's';
                        }
                        char
                    })
                    .collect();
                acc.push(row);
                acc
            });

        Self {
            start,
            rows: floor.len(),
            cols: floor[0].len(),
            floor,
        }
    }

    fn walk(&mut self) {
        let mut val = 1;
        let mut cycles = HashSet::new();
        let mut i = self.start[0];
        let mut j = self.start[1];
        let mut guard = Guard::Up;
        while let Some(char) = self.peek_next([i, j], &guard) {
            match char {
                '#' => {
                    guard = guard.get_next_guard();
                    continue;
                }
                '.' | 'x' | 's' => {
                    [i, j] = self.step([i, j], &guard).unwrap();
                    if char == '.' {
                        val += 1;
                        self.floor[i][j] = 'x';
                    }
                    if let Some(coord) = self.check_cycle(&[i, j], &guard) {
                        cycles.insert(coord);
                    }
                }
                e => panic!("Invalid character on board! {}", e),
            }
        }
        // dbg!(format!("x: {}, y: {}", j, i));

        println!("Value 1: {}", val);
        println!("Value 2: {}", cycles.len());
    }

    fn step(&self, idx: [usize; 2], guard: &Guard) -> Option<[usize; 2]> {
        let dir = guard.to_dir();
        let i = idx[0] as isize + dir[0];
        let j = idx[1] as isize + dir[1];
        if self.in_bounds(i, j) {
            Some([i as usize, j as usize])
        } else {
            None
        }
    }

    fn peek_next(&self, idx: [usize; 2], guard: &Guard) -> Option<char> {
        self.step(idx, guard)
            .map(|next| self.floor[next[0]][next[1]])
    }

    fn in_bounds(&self, i: isize, j: isize) -> bool {
        !(i < 0 || i as usize == self.rows || j < 0 || j as usize == self.cols)
    }

    fn check_cycle(&mut self, start: &[usize; 2], guard: &Guard) -> Option<[usize; 2]> {
        let mut ret = None;
        if self.peek_next(*start, guard).is_none() {
            return ret;
        }
        let next = self.step(*start, guard).unwrap();
        let saved = self.floor[next[0]][next[1]];
        if saved == 's' {
            return ret;
        }
        let mut i = start[0];
        let mut j = start[1];
        let mut stops = HashSet::new();
        self.floor[next[0]][next[1]] = '#';
        let mut next_guard = guard.get_next_guard();
        while let Some(char) = self.peek_next([i, j], &next_guard) {
            match char {
                '#' => {
                    let state = State {
                        pos: [i, j],
                        dir: next_guard.clone(),
                    };
                    if stops.get(&state).is_some() {
                        ret = Some(next);
                        break;
                    } else {
                        stops.insert(state);
                    }
                    next_guard = next_guard.get_next_guard();
                }
                's' | '.' | 'x' => {
                    [i, j] = self.step([i, j], &next_guard).unwrap();
                }
                e => panic!("Invalid character on board during cycle check! {}", e),
            }
        }

        self.floor[next[0]][next[1]] = saved;
        // if ret.is_some() {
        //     dbg!(&ret);
        // }
        ret
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Guard {
    Up,
    Right,
    Down,
    Left,
}

impl Guard {
    fn to_dir(&self) -> [isize; 2] {
        match self {
            Self::Up => [-1, 0],
            Self::Right => [0, 1],
            Self::Down => [1, 0],
            Self::Left => [0, -1],
        }
    }
    fn get_next_guard(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    pos: [usize; 2],
    dir: Guard,
}
