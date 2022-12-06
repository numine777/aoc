use anyhow::Result;
use std::collections::hash_set::HashSet;

fn check_packet(input: &str, len: usize) -> bool {
    return HashSet::<char>::from_iter(input.chars()).len() == len;
}

fn find_first_packet_marker(input: &String, len: usize) -> usize {
    let mut idx = 0;
    for (i, _) in input.chars().enumerate() {
        if check_packet(&input[i..=i+len-1], len) {
            idx = i + len;
            break;
        }
    }
    return idx;
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input6.prod")?;

    let ans_one = find_first_packet_marker(&input, 4);
    let ans_two = find_first_packet_marker(&input, 14);

    println!("Part one: {:?}", ans_one);
    println!("Part two: {:?}", ans_two);

    Ok(())
}
