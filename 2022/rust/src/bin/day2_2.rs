use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input2_1.test")?;

    let parsed_lines: usize = input.lines()
        .map(|line| {
            let result = match line {
                "A X" => 3,
                "A Y" => 1,
                "A Z" => 2,
                "B X" => 1,
                "B Y" => 2,
                "B Z" => 3,
                "C X" => 2,
                "C Y" => 3,
                "C Z" => 1,
                _ => 0,
            };
            let extra_points = match line.split(" ").collect::<Vec<&str>>()[1] {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };
            return result + extra_points;
        }).sum();

    println!("{:?}", parsed_lines);

    return Ok(());
}
