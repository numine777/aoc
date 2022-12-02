use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input1_1.test")?;
    let mut parsed_lines: Vec<usize> = input
        .split("\n\n")
        .map(|lines| lines.lines().flat_map(|line| line.parse::<usize>()).sum())
        .collect();

    parsed_lines.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", parsed_lines[0]);
    println!("Part 2: {:?}", parsed_lines.iter().take(3).sum::<usize>());

    return Ok(());
}
