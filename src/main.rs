use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

mod day01;
mod day02;

#[derive(Parser)]
struct Cli {
    day: i32,
    path: PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let file = File::open(args.path).unwrap();
    let lines = BufReader::new(file).lines();
    let result: (i32, i32) = match args.day {
        1 =>  day01::day01(lines),
        2 =>  day02::day02(lines),
        _ => panic!()
    };
    println!("{:?}", result)
}