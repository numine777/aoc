use anyhow::Result;

fn find_full_overlap(line: &str) -> usize {
    let ranges: Vec<&str> = line.split(",").collect();
    let range_one: Vec<usize> = ranges[0].split("-").map(|char| char.parse::<usize>().unwrap()).collect();
    let range_two: Vec<usize> = ranges[1].split("-").map(|char| char.parse::<usize>().unwrap()).collect();    
    return if range_one[0] <= range_two[0] && range_one[1] >= range_two[1] {
        1
    } else if range_one[0] >= range_two[0] && range_one[1] <= range_two[1] {
        1
    } else {
        0
    };
}

fn find_any_overlap(line: &str) -> usize {
    let ranges: Vec<&str> = line.split(",").collect();
    let range_one: Vec<usize> = ranges[0].split("-").map(|char| char.parse::<usize>().unwrap()).collect();
    let range_two: Vec<usize> = ranges[1].split("-").map(|char| char.parse::<usize>().unwrap()).collect();    
    return if range_one[0] <= range_two[0] && range_one[0] >= range_two[1] {
        1
    } else if range_one[1] >= range_two[0] && range_one[1] <= range_two[1] {
        1
    } else if range_two[0] <= range_one[0] && range_two[0] >= range_one[1] {
        1
    } else if range_two[1] >= range_one[0] && range_two[1] <= range_one[1] {
        1
    } else {
        0
    };
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input4_1.prod")?;

    let part_one: usize = input.lines().map(|line| find_full_overlap(line)).sum();
    let part_two: usize = input.lines().map(|line| find_any_overlap(line)).sum();

    println!("Part one: {:?}", part_one);
    println!("Part two: {:?}", part_two);

    return Ok(());
}
