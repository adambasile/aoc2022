use std::collections::VecDeque;

use regex::Regex;

pub fn day05(lines: Vec<String>) -> (String, String) {
    let mut stacks = Vec::<VecDeque<char>>::new();
    let num_stacks = lines[0].len() / 4 + 1;
    for _ in 0..(num_stacks) { stacks.push(VecDeque::new()) }
    // build up stacks
    for line in lines.iter().filter(|line| line.contains('[')) {
        for i in 0..num_stacks {
            let label = line.chars().nth(i * 4 + 1).unwrap();
            if label != ' ' { stacks.get_mut(i).unwrap().push_back(label) }
        }
    }
    let mut partone_stacks = stacks.clone();
    let mut parttwo_stacks = stacks;
    // move things around
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in lines.into_iter().filter(|line| re.is_match(line)) {
        let caps = re.captures(&line).unwrap();
        let num_crates = caps[1].parse::<usize>().unwrap();
        let source = caps[2].parse::<usize>().unwrap() - 1;
        let dest = caps[3].parse::<usize>().unwrap() - 1;
        // part one
        for _ in 0..num_crates {
            let crate_to_move = partone_stacks.get_mut(source).unwrap().pop_front().unwrap();
            partone_stacks.get_mut(dest).unwrap().push_front(crate_to_move)
        }
        // part two
        let mut crates_to_move = Vec::new();
        for _ in 0..num_crates {
            crates_to_move.push(parttwo_stacks.get_mut(source).unwrap().pop_front().unwrap())
        }
        while !crates_to_move.is_empty() {
            parttwo_stacks.get_mut(dest).unwrap().push_front(crates_to_move.pop().unwrap())
        }
    }
    (top_crates(&partone_stacks), top_crates(&parttwo_stacks))
}

fn top_crates(stacks: &Vec<VecDeque<char>>) -> String {
    let mut out = String::new();
    for stack in stacks { out.push(*stack.get(0).unwrap()) }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_05() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day05.txt");
        let lines = read_lines_from_file(filename);
        let (partone, parttwo) = day05(lines);
        assert_eq!(&partone, "RTGWZTHLD");
        assert_eq!(&parttwo, "STHGRZZFR");
    }
}
