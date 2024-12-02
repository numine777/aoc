use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_to_string("./input1.txt")?;
    let mut list_one = vec![];
    let mut list_two = vec![];
    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace();
        list_one.push(nums.next().unwrap().parse::<usize>().unwrap());
        list_two.push(nums.next().unwrap().parse::<usize>().unwrap());
    });
    part_one(&mut list_one, &mut list_two)?;
    part_two(list_one, list_two)?;

    Ok(())
}

fn part_one(list_one: &mut Vec<usize>, list_two: &mut Vec<usize>) -> Result<()> {
    list_one.sort();
    list_two.sort();
    // println!("{:?}", list_one);
    // println!("{:?}", list_two);
    let total_dist = list_one.iter().enumerate().fold(0, |acc, (idx, val)| {
        let val_two = list_two[idx];
        if val > &val_two {
            acc + (val - val_two)
        } else {
            acc + (val_two - val)
        }
    });
    println!("Total: {}", total_dist);
    Ok(())
}

fn part_two(list_one: Vec<usize>, list_two: Vec<usize>) -> Result<()> {
    let mut counter = HashMap::new();
    let similarity_score = list_one.iter().fold(0, |acc, val| {
        if !counter.contains_key(val) {
            let count = list_two.iter().filter(|val_two| *val_two == val).count();
            counter.insert(val, count);
        }
        acc + (val * counter.get(val).unwrap())
    });
    // println!("{:?}", counter);
    println!("Similarity score: {}", similarity_score);
    Ok(())
}
