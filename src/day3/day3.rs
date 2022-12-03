use crate::http_get::get_aoc_day_data;

use std::{cmp, collections::HashMap, error::Error};

pub fn day_three(day: i32) {
    let data = get_aoc_day_data(&day.to_string());

    let data = &parse_data(data.to_owned()).unwrap();

    println!("Day 3 part 2: {:?}", day_three_part_two(data.to_owned()));

    // println!("Day 3 part 1: {:?}", day_three_part_one(data.to_owned()));
}

fn day_three_part_one(data: Vec<String>) -> i32 {
    let mut summed_priority = 0;
    for rucksack in data {
        summed_priority += parse_rucksack(rucksack);
    }

    return summed_priority;
}

fn day_three_part_two(data: Vec<String>) -> i32 {
    let mut summed_priority = 0;
    let mut i = 1;
    for (idx, _rucksack) in data.iter().enumerate() {
        if i == 1 {
            if idx == data.len() {
                break;
            }
            let row_one = &data[idx];
            let row_two = &data[idx + 1];
            let row_three = &data[idx + 2];
            summed_priority +=
                parse_rucksack_part_two(format!("{}{}{}", &row_one, &row_two, &row_three));
        }

        i += 1;

        if i == 4 {
            i = 1;
        }
    }

    return 1;
}

fn parse_rucksack_part_two(concated: String) -> i32 {
    let characters = concated.chars();

    // let asd: Vec<char> = characters.to_owned().map(|elf| elf).collect();

    let mut map: HashMap<String, i32> = HashMap::new();

    for c in characters {
        let mut old_value = map.get(&c.to_string()).to_owned();
        let mut value = 1;
        match old_value {
            Some(v) => {
                let mut paska = v.to_owned();
                return paska += 1;
            },
            _ => 1,

        };

        map.insert(c.to_string(), 1);
    }

    // println!("asd: {:?}", characters);

    return 1;
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

    // let mut priority = HashMap::new();

    // priority.insert(String::from("a"), 1);
    // priority.insert(String::from("b"), 2);
    // priority.insert(String::from("c"), 3);
    // priority.insert(String::from("d"), 4);
    // priority.insert(String::from("e"), 5);
    // priority.insert(String::from("f"), 6);
    // priority.insert(String::from("g"), 7);
    // priority.insert(String::from("h"), 8);
    // priority.insert(String::from("i"), 9);
    // priority.insert(String::from("j"), 10);
    // priority.insert(String::from("k"), 11);
    // priority.insert(String::from("l"), 12);
    // priority.insert(String::from("m"), 13);
    // priority.insert(String::from("n"), 14);
    // priority.insert(String::from("o"), 15);
    // priority.insert(String::from("p"), 16);
    // priority.insert(String::from("q"), 17);
    // priority.insert(String::from("r"), 18);
    // priority.insert(String::from("s"), 19);
    // priority.insert(String::from("t"), 20);
    // priority.insert(String::from("u"), 21);
    // priority.insert(String::from("v"), 22);
    // priority.insert(String::from("w"), 23);
    // priority.insert(String::from("x"), 24);
    // priority.insert(String::from("y"), 25);
    // priority.insert(String::from("z"), 26);

    // priority.insert(String::from("A"), 27);
    // priority.insert(String::from("B"), 28);
    // priority.insert(String::from("C"), 29);
    // priority.insert(String::from("D"), 30);
    // priority.insert(String::from("E"), 31);
    // priority.insert(String::from("F"), 32);
    // priority.insert(String::from("G"), 33);
    // priority.insert(String::from("H"), 34);
    // priority.insert(String::from("I"), 35);
    // priority.insert(String::from("J"), 36);
    // priority.insert(String::from("K"), 37);
    // priority.insert(String::from("L"), 38);
    // priority.insert(String::from("M"), 39);
    // priority.insert(String::from("N"), 40);
    // priority.insert(String::from("O"), 41);
    // priority.insert(String::from("P"), 42);
    // priority.insert(String::from("Q"), 43);
    // priority.insert(String::from("R"), 44);
    // priority.insert(String::from("S"), 45);
    // priority.insert(String::from("T"), 46);
    // priority.insert(String::from("U"), 47);
    // priority.insert(String::from("V"), 48);
    // priority.insert(String::from("W"), 49);
    // priority.insert(String::from("X"), 50);
    // priority.insert(String::from("Y"), 51);
    // priority.insert(String::from("Z"), 52);

    // return priority;
}

fn parse_data(data: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut parsed: Vec<String> = Vec::new();

    for line in data.lines() {
        let line = line.trim_end();
        parsed.push(line.to_string());
    }
    Ok(parsed)
}
