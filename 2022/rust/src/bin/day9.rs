use std::collections::HashSet;

use anyhow::Result;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() -> Result<()> {
    let mut seen = HashSet::from([(0, 0)]);
    let mut head = (0, 0);
    let mut knots = vec![(0, 0); 9];
    for (dir, mut k) in std::fs::read_to_string("./input9.prod")?
        .lines()
        .map(|line| {
            let (dir, k) = line.split_once(" ").unwrap();
            let direction = match dir {
                "R" => DIRECTIONS[2],
                "L" => DIRECTIONS[3],
                "U" => DIRECTIONS[0],
                "D" => DIRECTIONS[1],
                _ => (0, 0),
            };
            return (direction, k.parse::<isize>().unwrap());
        })
    {
        while k > 0 {
            let (x, y) = dir;
            head = (head.0 + x, head.1 + y);
            let mut next = head;
            knots = knots
                .iter()
                .enumerate()
                .map(|(i, knot)| {
                    let diff_x = next.0 - knot.0;
                    let diff_y = next.1 - knot.1;
                    let mut new_knot = *knot;
                    if diff_x > 1 || diff_y > 1 || diff_x < -1 || diff_y < -1 {
                        if diff_x != 0 {
                            if diff_x > 0 {
                                new_knot = (new_knot.0 + 1, new_knot.1);
                            } else {
                                new_knot = (new_knot.0 - 1, new_knot.1);
                            }
                        } 
                        if diff_y != 0 {
                            if diff_y > 0 {
                                new_knot = (new_knot.0, new_knot.1 + 1);
                            } else {
                                new_knot = (new_knot.0, new_knot.1 - 1);
                            }
                        }
                        if i == 8 {
                            seen.insert(new_knot);
                        }
                    }
                    next = new_knot;
                    new_knot
                })
                .collect();
            k -= 1;
        }
    }
    println!("Part one: {:?}", seen.len());
    Ok(())
}
