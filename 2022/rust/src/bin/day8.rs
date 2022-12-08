use anyhow::Result;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn check_edge(max_x: i32, max_y: i32, x: i32, y: i32) -> bool {
    return y == 0 || x == 0 || y == max_y || x == max_x;
}

fn check_tree_vis(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
    let mut visible = false;

    let height = grid[i][j];
    let max_x = grid[i].len() - 1;
    let max_y = grid.len() - 1;

    for (v, h) in DIRECTIONS {
        let mut k = 0;
        loop {
            let x = h * k + j as i32;
            let y = v * k + i as i32;
            let next_height = grid[y as usize][x as usize];
            if k > 0 && next_height >= height {
                break;
            }
            if check_edge(max_x as i32, max_y as i32, x, y) {
                visible = true;
                break;
            }
            k += 1;
        }
        if visible {
            break;
        }
    }

    return visible as i32;
}

fn find_scenic_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
    let height = grid[i][j];
    let max_x = grid[i].len() - 1;
    let max_y = grid.len() - 1;

    let mut total = 1;

    for (v, h) in DIRECTIONS {
        let mut k = 0;
        loop {
            let x = h * k + j as i32;
            let y = v * k + i as i32;
            let next_height = grid[y as usize][x as usize];
            if k > 0 && next_height >= height || check_edge(max_x as i32, max_y as i32, x, y) {
                break;
            }
            k += 1;
        }
        total *= k;
    }

    return total;
}

fn main() -> Result<()> {
    let mut count = 0;
    let grid: Vec<Vec<u32>> = std::fs::read_to_string("./input8.prod")?
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut totals = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            count += check_tree_vis(&grid, i, j);
            totals.push(find_scenic_score(&grid, i, j));
        }
    }

    println!("Part one: {:?}", count);
    println!("Part two: {:?}", totals.iter().max().unwrap());

    return Ok(());
}
