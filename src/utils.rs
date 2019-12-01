use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn read_i32_from_file<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
