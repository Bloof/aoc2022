use crate::http_get::get_aoc_day_data;

use std::{cmp, collections::{HashMap}, error::Error};

pub fn day_three(day: i32) {
    let data = get_aoc_day_data(&day.to_string());

    let data = &parse_data(data.to_owned()).unwrap();

    println!("Day 3 part 1: {:?}", day_three_part_one(data.to_owned()));

    println!("Day 3 part 2: {:?}", day_three_part_two(data.to_owned()));

}

fn day_three_part_one(data: Vec<String>) -> i32 {
    let mut summed_priority = 0;
    for rucksack in data {
        summed_priority += parse_rucksack(rucksack);
    }

    return summed_priority;
}

fn day_three_part_two(data: Vec<String>) -> i32 {

    let priorities = get_priority_board();

    // Find the badge priorities for each group
    let mut total_priority = 0;
    for group in data.chunks(3) {
        let priority = find_badge_priorities(&priorities, group);
        total_priority += priority;
    }

    return total_priority;
}

fn find_badge_priorities(priorities: &HashMap<String, i32>, rucksacks: &[String]) -> i32 {
    // Count the frequency of each item type in the rucksacks
    let mut counts: HashMap<char, i32> = HashMap::new();
    for rucksack in rucksacks {
        for c in rucksack.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let common_char = common_char(rucksacks);

    // Look up the priority for the most frequent item type
    match priorities.get(&common_char.to_string()) {
        Some(priority) => *priority,
        None => 0,
    }
}
fn common_char(text: &[String]) -> char {
    let mut common_char = ' ';
    let mut common_char_count = 0;

    for row in text {
        for ch in row.chars() {
            if ch.is_alphabetic() {
                let mut count = 0;
                for row in text {
                    if row.contains(ch) {
                        count += 1;
                    }
                }
                if count == text.len() {
                    common_char = ch;
                    common_char_count = count;
                    break;
                }
            }
        }
    }

    if common_char_count == text.len() {
        common_char
    } else {
        ' '
    }

}


fn parse_rucksack(data: String) -> i32 {
    let mut priority = 0;
    let priority_board = get_priority_board();

    let cut_off_point = data.len() / 2;
    let (first_item, second_item) = data.split_at(cmp::min(cut_off_point, data.len()));

    let first_item: Vec<char> = first_item.chars().collect();
    let second_item_vec: Vec<char> = second_item.chars().collect();

    for character in first_item.to_owned() {
        let common_item_type = second_item.find(character);
        match common_item_type {
            Some(common) => {
                priority = priority_board
                    .get(&second_item_vec[common].to_string())
                    .copied()
                    .unwrap_or(0)
            }
            _ => continue,
        };
    }
    return priority;
}

fn get_priority_board() -> HashMap<String, i32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut alphabet_map = HashMap::new();

    for (i, c) in alphabet.chars().enumerate() {
        alphabet_map.insert(c.to_string(), i as i32 + 1);
    }
    return alphabet_map;
}

fn parse_data(data: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut parsed: Vec<String> = Vec::new();

    for line in data.lines() {
        let line = line.trim_end();
        parsed.push(line.to_string());
    }
    Ok(parsed)
}
