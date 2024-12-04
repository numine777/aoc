use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_to_string("./input4.txt")?;
    part_one(&input)?;
    part_two(&input)?;
    Ok(())
}

fn part_one(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let target = "XMAS";

    let dirs: &[&[isize]] = &[
        &[0, 1],
        &[0, -1],
        &[1, 0],
        &[-1, 0],
        &[1, 1],
        &[-1, 1],
        &[1, -1],
        &[-1, -1],
    ];

    dfs(&grid, target, dirs);
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let target = "MAS";

    xsearch(&grid, target);
    Ok(())
}

fn is_valid(idx: &[isize], max_l: usize, max_h: usize) -> bool {
    idx[0] >= 0 && idx[0] < max_l as isize && idx[1] >= 0 && idx[1] < max_h as isize
}

fn dfs(grid: &Vec<Vec<char>>, target: &str, dirs: &[&[isize]]) {
    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            for dir in dirs {
                if target.starts_with(grid[i][j]) {
                    count += walk(grid, &target[1..], &[i, j], dir);
                }
            }
        }
    }
    println!("Count 1: {}", count);
}

fn walk(grid: &[Vec<char>], target: &str, idx: &[usize], dir: &[isize]) -> usize {
    if target.is_empty() {
        return 1;
    }
    let m = grid.len();
    let n = grid[0].len();
    let x = idx[0] as isize + dir[0];
    let y = idx[1] as isize + dir[1];
    if !is_valid(&[x, y], m, n) {
        return 0;
    }
    if !target.starts_with(grid[x as usize][y as usize]) {
        return 0;
    }
    walk(grid, &target[1..], &[x as usize, y as usize], dir)
}

fn xsearch(grid: &[Vec<char>], target: &str) {
    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'A' && i >= 1 && i < m - 1 && j >= 1 && j < n - 1 {
                count += evaluate(grid, &[i, j], target);
            }
        }
    }
    println!("Count 2: {}", count);
}

fn evaluate(grid: &[Vec<char>], idx: &[usize], target: &str) -> usize {
    let mut count = 0;
    if grid[idx[0] - 1][idx[1] - 1] == 'M' {
        count += walk(grid, &target[1..], &[idx[0] - 1, idx[1] - 1], &[1, 1]);
    }
    if grid[idx[0] - 1][idx[1] + 1] == 'M' {
        count += walk(grid, &target[1..], &[idx[0] - 1, idx[1] + 1], &[1, -1]);
    }
    if grid[idx[0] + 1][idx[1] - 1] == 'M' {
        count += walk(grid, &target[1..], &[idx[0] + 1, idx[1] - 1], &[-1, 1]);
    }
    if grid[idx[0] + 1][idx[1] + 1] == 'M' {
        count += walk(grid, &target[1..], &[idx[0] + 1, idx[1] + 1], &[-1, -1]);
    }
    if count == 2 {
        1
    } else {
        0
    }
}
