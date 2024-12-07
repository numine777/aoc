use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input7.txt").unwrap();
    let ans = input.lines().fold(0, |acc, line| {
        let (res, remainder) = line.split_once(':').unwrap();
        let target: isize = res.parse().unwrap();
        let nums: Vec<isize> = remainder
            .split(' ')
            .filter_map(|val| match val.parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            })
            .collect();

        if check_combinations(&nums, target) {
            acc + target
        } else {
            acc
        }
    });
    println!("Ans 1: {}", ans);
}

fn check_combinations(nums: &[isize], target: isize) -> bool {
    // Early return for empty case
    if nums.is_empty() {
        return target == 0;
    }

    // Use dynamic programming to store intermediate results
    let mut seen = HashSet::new();
    let mut stack = vec![(0, 0)]; // (index, current_value)

    while let Some((idx, current)) = stack.pop() {
        if idx == nums.len() {
            if current == target {
                return true;
            }
            continue;
        }

        // Create unique state for memoization
        let state = (idx, current);
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);

        let num = nums[idx];

        // Try addition if it won't exceed target
        if current + num <= target {
            stack.push((idx + 1, current + num));
        }

        // Try multiplication only if current is not 0 and won't exceed target
        if current != 0 && current * num <= target {
            stack.push((idx + 1, current * num));
        }

        let concated = (format!("{}", current) + &format!("{}", num)).parse::<isize>().unwrap();
        if concated <= target {
            stack.push((idx + 1, concated));
        }
    }

    false
}
