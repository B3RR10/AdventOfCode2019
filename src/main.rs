use std::env;

mod day1;
mod day2;
mod day3;
mod utils;

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
            let input: Vec<i32> = utils::read_int_from_file(input, '\n');
            println!("Part 1: {}", day1::part1(&input));
            println!("Part 2: {}", day1::part2(&input));
        }
        "day2" => {
            let input: Vec<i32> = utils::read_int_from_file(input, ',');
            println!("Part 1: {}", day2::part1(&input));
            println!("Part 2: {}", day2::part2(&input));
        }
        "day3" => {
            let input: Vec<String> = utils::read_lines(input);
            println!("Part 1: {}", day3::part1(&input));
        }
        _ => println!("Invalid day. Insert day in the form dayX"),
    }
}
