use std::fs::File;
use std::io::{prelude::Read, BufRead, BufReader};
use std::path::Path;

pub fn read_int_from_file<P>(filename: P, separator: char) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let mut buff = String::new();
    BufReader::new(file)
        .read_to_string(&mut buff)
        .expect("Can't read file");
    buff.trim()
        .split(separator)
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect::<Vec<String>>()
}
