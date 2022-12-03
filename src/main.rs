use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod dayone;
mod daytwo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse().unwrap();
    let filepath = Path::new(&args[2]);

    let file = File::open(filepath).unwrap();
    let lines = BufReader::new(file).lines();
    let result = match day {
        1 => format!("{:?}", dayone::day01(lines)),
        2 => format!("{:?}", daytwo::day02(lines)),
        _ => panic!()
    };
    println!("{}", result)
}