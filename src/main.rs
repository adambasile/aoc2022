use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

mod dayone;
mod daytwo;

#[derive(Parser)]
struct Cli {
    day: i32,
    path: PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let file = File::open(args.path).unwrap();
    let lines = BufReader::new(file).lines();
    let result = match args.day {
        1 => format!("{:?}", dayone::day01(lines)),
        2 => format!("{:?}", daytwo::day02(lines)),
        _ => panic!()
    };
    println!("{}", result)
}