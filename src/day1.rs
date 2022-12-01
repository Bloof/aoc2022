use std::{cmp::Ordering, error::Error};

use crate::http_get::get_aoc_day_data;


pub fn day_one(day: i32) {

    let data = get_aoc_day_data(&day.to_string());

    println!(
        "Day 1 part 1: {:?}",
        day_one_part1(&parse_data(data.to_owned()).unwrap())
    );

    println!(
        "Day 1 part 2: {:?}",
        day_one_part2(&parse_data(data.to_owned()).unwrap())
    );

}

fn day_one_part1(data: &[Vec<u64>]) -> u64 {
    return data.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0);
}

fn day_one_part2(data: &[Vec<u64>]) -> u64 {
    let mut summed_elfs: Vec<u64> = data.iter().map(|elf| elf.iter().sum::<u64>()).collect();

    let index1 = get_elf_with_top_calories(&summed_elfs);
    let top_one_elf = summed_elfs[index1.unwrap()];
    summed_elfs.remove(index1.unwrap());

    let index2 = get_elf_with_top_calories(&summed_elfs);
    let top_two_elf = summed_elfs[index2.unwrap()];
    summed_elfs.remove(index2.unwrap());

    let index3 = get_elf_with_top_calories(&summed_elfs);
    let top_three_elf = summed_elfs[index3.unwrap()];

    return top_one_elf + top_two_elf + top_three_elf;
}

fn get_elf_with_top_calories(summed_elfs: &Vec<u64>) -> Option<usize> {
    return summed_elfs
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index);
}

fn parse_data(data: String) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let mut parsed = vec![];
    let mut last = 0;

    for line in data.lines() {
        let line = line.trim_end();
        if line.is_empty() || parsed.is_empty() {
            last = parsed.len();
            parsed.push(vec![]);

            if line.is_empty() {
                continue;
            }
        }

        parsed[last].push(line.parse()?);
    }
    Ok(parsed)
}
