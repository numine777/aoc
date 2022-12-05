use anyhow::Result;

fn ans_to_val(ans: u32) -> u32 {
    match ans {
        97..=122 => ans - 96,
        65..=90 => ans - 38,
        _ => 0,
    }
}

fn find_duplicate_value(line: &str) -> u32 {
    let mid = line.len() / 2;

    let one = line.get(..mid).unwrap();
    let two = line.get(mid..).unwrap();

    let mut ans = 0;
    for char_one in one.chars() {
        for char_two in two.chars() {
            if char_one == char_two {
                ans = char_one as u32;
                break;
            }
        }
        if ans > 0 {
            break;
        }
    }

    return ans_to_val(ans);
}

fn find_group_name(lines: &[&str]) -> u32 {
    let one = lines[0];
    let two = lines[1];
    let three = lines[2];

    let mut ans = 0;
    for char_one in one.chars() {
        for char_two in two.chars() {
            if char_one == char_two {
                for char_three in three.chars() {
                    if char_three == char_one {
                        ans = char_three as u32;
                        break;
                    }
                }
                if ans > 0 {
                    break;
                }
            }
            if ans > 0 {
                break;
            }
        }
    }
    return ans_to_val(ans);
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input3_1.prod")?;

    let parsed: u32 = input.lines().map(|line| find_duplicate_value(line)).sum();

    let parsed_2: u32 = input.lines().collect::<Vec<&str>>().chunks(3).map(|chunk| find_group_name(chunk)).sum();

    println!("Part 1: {:?}", parsed);
    println!("Part 2: {:?}", parsed_2);

    return Ok(());
}
