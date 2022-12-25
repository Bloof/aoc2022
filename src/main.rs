use std::io;

use crate::{day1::day1::day_one, day2::day2::day_two, day3::day3::day_three, day4::day4::day_four};

mod http_get;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Type in which day you want to execute:");
    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    let day: i32 = day.trim().parse().unwrap();

    let day_to_run: fn(day: i32) = match day {
        1 => day_one,
        2 => day_two,
        3 => day_three,
        4 => day_four,
        _ => panic!("Day not implemented")
    };

    day_to_run(day);

}