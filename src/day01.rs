use std::collections::binary_heap::BinaryHeap;

pub fn day01(lines: Vec<String>) -> (i32, i32) {
    // initialise with three empty elves so the unwraps below are always fine
    let mut elves = BinaryHeap::from([0, 0, 0]);
    lines
        .split(|line| line.is_empty())
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
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_01_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day01test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day01(lines), (20, 44));
    }

    #[test]
    fn test_day_01() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day01.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day01(lines), (69528, 206152));
    }
}
