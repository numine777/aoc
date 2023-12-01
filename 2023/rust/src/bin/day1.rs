use std::collections::HashMap;

use anyhow::Result;

fn find_first_num(map: &HashMap<&str, &str>, line: &str) -> String {
    let mut new_word = String::new();
    for char in line.chars() {
        if char.is_ascii_digit() {
            return char.to_string();
        } else {
            new_word.push(char);
            for (key, value) in map {
                if new_word.contains(key) {
                    return value.to_string();
                }
            }
        }
    }
    return String::new();
}

fn find_last_num(map: &HashMap<&str, &str>, line: &str) -> String {
    let mut new_word = String::new();
    for char in line.chars().rev() {
        if char.is_ascii_digit() {
            return char.to_string();
        } else {
            new_word = char.to_string() + &new_word;
            for (key, value) in map {
                if new_word.contains(key) {
                    return value.to_string();
                }
            }
        }
    }
    return String::new();
}

fn main() -> Result<()> {
    let mut num_words = HashMap::new();
    num_words.insert("one", "1");
    num_words.insert("two", "2");
    num_words.insert("three", "3");
    num_words.insert("four", "4");
    num_words.insert("five", "5");
    num_words.insert("six", "6");
    num_words.insert("seven", "7");
    num_words.insert("eight", "8");
    num_words.insert("nine", "9");
    let input = std::fs::read_to_string("day1.txt")?
        .lines()
        .map(|line| {
            let first_num = find_first_num(&num_words, line);
            let last_num = find_last_num(&num_words, line);
            println!("{} + {}", first_num, last_num);
            (first_num + &last_num).parse::<i32>().unwrap()
        })
        .sum::<i32>();
    println!("{}", input);
    Ok(())
}
