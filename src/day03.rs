use std::collections::HashSet;

fn priority(c: &char) -> i32 {
    match c.is_uppercase() {
        false => (*c as i32) - ('a' as i32) + 1,
        true => (*c as i32) - ('A' as i32) + 27,
    }
}

pub fn day03(lines: Vec<String>) -> (i32, i32) {
    let mut partone = 0;
    let mut parttwo = 0;

    for line in &lines {
        let (leftstr, rightstr) = line.split_at(line.len() / 2);
        let leftset: HashSet<_> = HashSet::from_iter(leftstr.chars());
        let rightset: HashSet<_> = HashSet::from_iter(rightstr.chars());
        let common_item: Vec<&char> = leftset.intersection(&rightset).collect();
        if common_item.len() != 1 { panic!() }
        for c in common_item { partone += priority(c) }
    }

    for elfgroup in lines.chunks(3) {
        let mut elfsets = elfgroup
            .iter()
            .map(|elf| elf.chars().collect::<HashSet<_>>());
        let mut common = elfsets.next().unwrap();
        for elf in elfsets {
            common = common.intersection(&elf).copied().collect();
        }
        if common.len() != 1 { panic!() }
        for c in common { parttwo += priority(&c) }
    }
    (partone, parttwo)
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_03() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day03.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day03(lines), (8202, 2864));
    }
}