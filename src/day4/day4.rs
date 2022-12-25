use std::ops::Range;

use crate::http_get::get_aoc_day_data;

pub fn day_four(day: i32) {
    let data = get_aoc_day_data(&day.to_string());

    let data = &parse_data(data.to_owned());

    println!("Day 4 part 1: {:?}", day_four_part_one(data.to_owned()));

    println!("Day 4 part 2: {:?}", day_four_part_two(data.to_owned()));
}

pub fn day_four_part_one(ranges: Vec<(Range<i32>, Range<i32>)>) -> i32 {
    let mut total_contained_ranges: i32 = 0;

    for (outer_range, inner_range) in ranges {
        if outer_range.start <= inner_range.start && outer_range.end >= inner_range.end {
            total_contained_ranges += 1;
        } else if inner_range.start <= outer_range.start && inner_range.end >= outer_range.end {
            total_contained_ranges += 1;
        }
    }
    return total_contained_ranges;
}

pub fn day_four_part_two(ranges: Vec<(Range<i32>, Range<i32>)>) -> i32 {
    let mut total_overlapping_ranges: i32 = 0;

    for (_i, (outer_range, inner_range)) in ranges.iter().enumerate() {
        if outer_range.start <= inner_range.end && outer_range.end >= inner_range.start {
            total_overlapping_ranges += 1;
        } else if inner_range.start <= outer_range.end && inner_range.end >= outer_range.start {
            total_overlapping_ranges += 1;
        }
    }
    return total_overlapping_ranges;
}

pub fn parse_data(data: String) -> Vec<(Range<i32>, Range<i32>)> {
    let mut ranges = Vec::new();

    for line in data.split("\n") {
        let mut range_strings = line.split(",");
        let range1: Vec<i32> = range_strings
            .next()
            .unwrap()
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect();

        let range2: Vec<i32> = range_strings
            .next()
            .unwrap()
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect();

        ranges.push((range1[0]..range1[1], range2[0]..range2[1]));
    }
    ranges
}
