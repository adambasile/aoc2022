use std::io::{BufReader, Lines, Read};
use std::collections::binary_heap::BinaryHeap;

pub fn day01<T: Read + Sized>(lines: Lines<BufReader<T>>) -> (i32, i32) {
    // initialise with three empty elves so the unwraps below are always fine
    let mut elves = BinaryHeap::from([0, 0, 0]);
    lines
        .filter_map(|result| result.ok())
        .collect::<Vec<String>>()
        .split(|line| line == "")
        .map(|elf_lines| {
            elf_lines
                .iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .for_each(|elf| elves.push(elf));

    let top_elf = elves.pop().unwrap();
    let top3_elves = top_elf + elves.pop().unwrap() + elves.pop().unwrap();
    (top_elf, top3_elves)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufRead;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_day_one() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day01test.txt");
        let file = File::open(filename).unwrap();
        let lines = BufReader::new(file).lines();
        assert_eq!(day01(lines), (20, 44));
    }
}
