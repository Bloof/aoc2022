use std::fs;

mod http_get;

fn main() {
    let data = http_get::get_aoc_day_data("5");
    println!("data: {:?}", data);
}


