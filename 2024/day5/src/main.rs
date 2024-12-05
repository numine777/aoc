use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_to_string("./input5.txt")?;
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let (defs, pages) = input.split_once("\n\n").unwrap();
    defs.lines().for_each(|line| {
        if line.contains('|') {
            let (key, val) = line.split_once('|').unwrap();
            if let Some(val_arr) = rules.get_mut(key) {
                val_arr.push(val);
            } else {
                rules.insert(key, vec![val]);
            }
        }
    });
    part_one(&rules, pages)?;
    part_two(&rules, pages)?;
    Ok(())
}

fn part_one(rules: &HashMap<&str, Vec<&str>>, pages: &str) -> Result<()> {
    let total = pages.lines().fold(0, |acc, line| {
        let mut ret = 0;
        let pages: Vec<&str> = line.split(',').collect();
        let mut seen = vec![];
        let mut rule_broken = false;
        pages.iter().enumerate().for_each(|(idx, val)| {
            if idx == pages.len() / 2 {
                ret = val.parse().unwrap();
            }
            if let Some(val_arr) = rules.get(val) {
                for seen_val in &seen {
                    if val_arr.contains(seen_val) {
                        rule_broken = true;
                        break;
                    }
                }
            }
            seen.push(val);
        });
        if rule_broken {
            acc
        } else {
            acc + ret
        }
    });

    println!("Total 1: {}", total);

    Ok(())
}

fn part_two(rules: &HashMap<&str, Vec<&str>>, pages: &str) -> Result<()> {
    let total = pages.lines().fold(0, |acc, line| {
        let pages: Vec<&str> = line.split(',').collect();
        let mut seen = vec![pages[0]];
        let mut changes_made = false;
        pages.iter().skip(1).for_each(|val| {
            let mut rule_broken = false;
            if let Some(val_arr) = rules.get(val) {
                let mut j = 0;
                while j < seen.len() {
                    let seen_val = seen[j];
                    if val_arr.contains(&seen_val) {
                        rule_broken = true;
                        changes_made = true;
                        seen.insert(j, val);
                        break;
                    }
                    j += 1;
                }
            }
            if !rule_broken {
                seen.push(val);
            }
        });
        if changes_made {
            let mid_seen: u64 = seen[seen.len() / 2].parse().unwrap();
            acc + mid_seen
        } else {
            acc
        }
    });

    println!("Total 2: {}", total);

    Ok(())
}
