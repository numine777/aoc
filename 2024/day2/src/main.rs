use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_to_string("./input2.txt")?;
    part_one(input.clone())?;
    part_two(input)?;
    Ok(())
}

fn part_one(input: String) -> Result<()> {
    let num_safe = input
        .lines()
        .filter(|line| {
            // dbg!(&line);
            let mut safe = true;
            let mut last = None;
            let mut increasing = None;
            for num in line.split_whitespace() {
                // dbg!(&num);
                let curr = num.parse::<isize>().unwrap();
                if let Some(prev) = last {
                    if let Some(inc) = increasing {
                        if inc && curr < prev || !inc && curr > prev {
                            safe = false;
                        }
                    }
                    if (curr - prev) > 0 {
                        increasing = Some(true);
                        if (curr - prev) < 1 || (curr - prev) > 3 {
                            safe = false;
                        }
                    } else {
                        increasing = Some(false);
                        if (prev - curr) < 1 || (prev - curr) > 3 {
                            safe = false;
                        }
                    }
                }
                last = Some(curr);
            }
            safe
        })
        .count();
    println!("Num safe 1: {}", num_safe);
    Ok(())
}

fn part_two(input: String) -> Result<()> {
    let num_safe = input.lines().filter(|line| eval_line(line)).count();
    println!("Num safe 2: {}", num_safe);
    Ok(())
}

fn check_safe(report: &Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    let mut last = report[0];
    for idx in 1..report.len() {
        if increasing {
            if last > report[idx] || (!matches!(report[idx] - last, 1..=3)) {
                return false;
            }
        }
        if !increasing {
            if last < report[idx] || (!matches!(last - report[idx], 1..=3)) {
                return false;
            }
        }
        last = report[idx];
    }
    true
}

fn eval_line(line: &str) -> bool {
    let report: Vec<i32> = line.split_whitespace().flat_map(|v| v.parse()).collect();
    if check_safe(&report) {
        true
    } else {
        for idx_to_drop in 0..report.len() {
            let mut modded_report = report.clone();
            modded_report.remove(idx_to_drop);
            if check_safe(&modded_report) {
                return true
            }
        }
        false
    }
}
