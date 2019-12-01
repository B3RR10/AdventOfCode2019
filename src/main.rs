use std::env;

mod day1;
mod utils;

use day1::*;
use utils::read_i32_from_file;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {
            let mut input: String = "input/".to_owned();
            input.push_str(&args[1][..]);
            run_day(&args[1][..], &input);
        }
        3 => run_day(&args[1][..], &args[1][..]),
        _ => println!("Usage:\n\tAOC <dayX> [inputfile]"),
    }
}

fn run_day(day: &str, input: &str) {
    match day {
        "day1" => {
            let input: Vec<i32> = read_i32_from_file(input);
            let part1: i32 = input.iter().map(|n| get_fuel(n.to_owned())).sum();
            println!("Part 1: {}", part1);
            let part2: i32 = input.iter().map(|n| get_fuel_recursive(n.to_owned())).sum();
            println!("Part 2: {}", part2);
        }
        _ => println!("Invalid day. Insert day in the form dayX"),
    }
}
