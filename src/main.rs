mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

extern crate dotenv;
extern crate core;

use dotenv::dotenv;
use std::env;

fn main() {
    let day_string = env::args().nth(1).or_else(|| {
        dotenv().ok();
        env::var("DAY").ok()
    }).unwrap_or("1".to_string());

    let day = day_string.parse::<i32>().expect("Wrong format for day variable");

    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        _ => {println!("Not yet implemented 😅")}
    }
}
