use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input10.prod")?;
    // let mut part_one = 0;
    let mut step_counter: isize = 1;
    let mut curr = 1;
    // let mut intervals = vec![220, 180, 140, 100, 60, 20];
    let mut hold_over = 0;
    let mut screen: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        // if intervals.is_empty() {
        //     break;
        // }
        // if step_counter <= *intervals.last().unwrap() {
        curr += hold_over;
        hold_over = 0;
        // }
        let idx = step_counter as usize / 40;
        if screen.len() <= idx {
            screen.push(Vec::new());
        }
        let mut current_idx = screen[idx].len() as isize;
        while step_counter % 40 >= current_idx {
            let c: char = if current_idx >= curr - 1 && current_idx <= curr + 1 {
                '#'
            } else {
                '.'
            };
            screen[idx].push(c);
            current_idx = screen[idx].len() as isize;
        }
        // if step_counter / intervals.last().unwrap() == 1 {
        //     let multiplier = intervals.pop().unwrap();
        //     part_one += multiplier * curr;
        //     // println!("{:?}, {:?}, {:?}", multiplier, curr, part_one);
        // }
        if line.starts_with("addx") {
            step_counter += 2;
            hold_over = line.split_once(" ").unwrap().1.parse::<isize>().unwrap();
        }
        if line.starts_with("noop") {
            step_counter += 1;
        }
    }

    // println!("Part one: {:?}", part_one);
    println!("Part two:\n");
    for vec in screen {
        println!("{:?}, {:?}", vec.iter().collect::<String>(), vec.len());
    }

    return Ok(());
}
