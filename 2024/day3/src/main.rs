use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_to_string("./input3.txt")?;
    part_one(&input)?;
    part_two(input)?;
    Ok(())
}

fn part_one(input: &str) -> Result<()> {
    let beginning = "mul(";
    let end = ')';
    let res = input.lines().fold(0, |mut acc, line| {
        let mut i = 0;
        while i < line.len() - 8 {
            let curr = line.chars().nth(i).unwrap();
            if curr == 'm' {
                let mut j = 0;
                let mut first_num_str = String::new();
                let mut second_num_str = String::new();
                let mut on_second_num = false;
                while j <= 11 {
                    j += 1;
                    let curr_char = line.chars().nth(i + j).unwrap();
                    if j < beginning.len() {
                        let beginning_char = beginning.chars().nth(j).unwrap();
                        if curr_char != beginning_char {
                            break;
                        }
                    } else {
                        if !on_second_num && first_num_str.len() < 3 && curr_char.is_numeric() {
                            first_num_str.push(curr_char);
                            continue;
                        }
                        if curr_char == ',' && !first_num_str.is_empty() && !on_second_num {
                            on_second_num = true;
                            continue;
                        }
                        if second_num_str.len() < 3 && curr_char.is_numeric() {
                            second_num_str.push(curr_char);
                            continue;
                        }
                        if !second_num_str.is_empty() && !first_num_str.is_empty() && curr_char == end {
                            dbg!(&first_num_str);
                            dbg!(&second_num_str);
                            let first_num: i64 = first_num_str.parse().unwrap();
                            let second_num: i64 = second_num_str.parse().unwrap();
                            acc += first_num * second_num;
                            break;
                        }
                        break;
                    }
                }

                i += j;
            } else {
                i += 1;
            }
        }
        acc
    });
    println!("Total 1: {}", res);
    Ok(())
}

fn part_two(input: String) -> Result<()> {
    let beginning = "mul(";
    let end = ')';
    let _do = "do()";
    let _dont = "don't()";
    let mut enabled = true;
    let res = input.lines().fold(0, |mut acc, line| {
        let mut i = 0;
        while i < line.len() - 8 {
            let curr = line.chars().nth(i).unwrap();
            if enabled && curr == 'm' {
                let mut j = 0;
                let mut first_num_str = String::new();
                let mut second_num_str = String::new();
                let mut on_second_num = false;
                while j <= 11 {
                    j += 1;
                    let curr_char = line.chars().nth(i + j).unwrap();
                    if j < beginning.len() {
                        let beginning_char = beginning.chars().nth(j).unwrap();
                        if curr_char != beginning_char {
                            break;
                        }
                    } else {
                        if !on_second_num && first_num_str.len() < 3 && curr_char.is_numeric() {
                            first_num_str.push(curr_char);
                            continue;
                        }
                        if curr_char == ',' && !first_num_str.is_empty() && !on_second_num {
                            on_second_num = true;
                            continue;
                        }
                        if second_num_str.len() < 3 && curr_char.is_numeric() {
                            second_num_str.push(curr_char);
                            continue;
                        }
                        if !second_num_str.is_empty() && !first_num_str.is_empty() && curr_char == end {
                            dbg!(&first_num_str);
                            dbg!(&second_num_str);
                            let first_num: i64 = first_num_str.parse().unwrap();
                            let second_num: i64 = second_num_str.parse().unwrap();
                            acc += first_num * second_num;
                            break;
                        }
                        break;
                    }
                }

                i += j;
            } else if curr == 'd' {
                dbg!(&line[i..=i+3]);
                dbg!(&line[i..=i+7]);
                if &line[i..=i+3] == _do {
                    enabled = true;
                    i += 3;
                } else if &line[i..=i+6] == _dont {
                    enabled = false;
                    i += 7;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        acc
    });
    println!("Total 2: {}", res);
    Ok(())
}
