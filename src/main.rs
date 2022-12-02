use std::io;

use crate::{day1::day1::day_one, day2::day2::day_two};


mod http_get;
mod day1;
mod day2;


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
        _ => panic!("Day not implemented")
    };

    day_to_run(day);

}