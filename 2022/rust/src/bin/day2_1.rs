use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input2_1.test")?;

    let parsed_lines: usize = input.lines()
        .map(|line| {
            let result = match line {
                "A X" => 3,
                "A Y" => 6,
                "A Z" => 0,
                "B X" => 0,
                "B Y" => 3,
                "B Z" => 6,
                "C X" => 6,
                "C Y" => 0,
                "C Z" => 3,
                _ => 0,
            };
            let extra_points = match line.split(" ").collect::<Vec<&str>>()[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            return result + extra_points;
        }).sum();

    println!("{:?}", parsed_lines);

    return Ok(());
}
