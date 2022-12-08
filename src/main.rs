use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

#[derive(Parser)]
struct Cli {
    day: i32,
    path: PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let lines = read_lines_from_file(args.path);
    match args.day {
        1 => println!("{:?}", day01::day01(lines)),
        2 => println!("{:?}", day02::day02(lines)),
        3 => println!("{:?}", day03::day03(lines)),
        4 => println!("{:?}", day04::day04(lines)),
        5 => println!("{:?}", day05::day05(lines)),
        6 => println!("{:?}", day06::day06(lines)),
        7 => println!("{:?}", day07::day07(lines)),
        8 => println!("{:?}", day08::day08(lines)),
        _ => panic!(),
    };
}

fn read_lines_from_file(path: PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .collect();
    lines
}
