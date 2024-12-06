use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input6.txt").unwrap();
    let mut start = [0, 0];
    let mut board = input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc, (i, line)| {
            let row: Vec<char> = line
                .chars()
                .enumerate()
                .map(|(j, char)| {
                    if char == '^' {
                        start = [i, j];
                    }
                    char
                })
                .collect();
            acc.push(row);
            acc
        });
    part_one(&mut board, &start);
}

fn part_one(board: &mut [Vec<char>], start: &[usize; 2]) {
    let mut val = 0;
    let mut i = start[0];
    let mut j = start[1];
    let mut guard = '^';
    let mut found_exit = false;
    while !found_exit {
        match guard {
            '^' => loop {
                if board[i-1][j] == '#' {
                    guard = '>';
                    break;
                }
                i -= 1;
                if board[i][j] == '.' {
                    val += 1;
                    board[i][j] = 'x';
                }
                if i as isize - 1 < 0 {
                    found_exit = true;
                    break;
                }
            },
            '>' => loop {
                if board[i][j+1] == '#' {
                    guard = 'v';
                    break;
                }
                j += 1;
                if board[i][j] == '.' {
                    val += 1;
                    board[i][j] = 'x';
                }
                if j + 1 == board[0].len() {
                    found_exit = true;
                    break;
                }
            },
            'v' => loop {
                i += 1;
                if board[i][j] == '.' {
                    val += 1;
                    board[i][j] = 'x';
                }
                if i + 1 == board.len() {
                    found_exit = true;
                    break;
                }
                if board[i+1][j] == '#' {
                    guard = '<';
                    break;
                }
            },
            '<' => loop {
                if board[i][j-1] == '#' {
                    guard = '^';
                    break;
                }
                j -= 1;
                if board[i][j] == '.' {
                    val += 1;
                    board[i][j] = 'x';
                }
                if j as isize - 1 < 0 {
                    found_exit = true;
                    break;
                }
            },
            _ => panic!("Invalid character for guard")
        }
    }
    dbg!(format!("x: {}, y: {}", j, i));

    println!("Value 1: {}", val);
}
