use std::collections::HashSet;

pub fn day03(lines: Vec<String>) -> (i32, i32) {
    let mut partone = 0;
    let mut parttwo = 0;

    for line in &lines {
        let (leftstr, rightstr) = line.split_at(line.len() / 2);
        let leftset: HashSet<_> = HashSet::from_iter(leftstr.chars());
        let rightset: HashSet<_> = HashSet::from_iter(rightstr.chars());

        let common_item: HashSet<_> = leftset.intersection(&rightset).collect();
        if common_item.len() != 1 { panic!() }
        for c in common_item { partone += value(c) }
    }

    for elfgroup in (&lines).chunks(3) {
        let mut elfsets = elfgroup
            .into_iter()
            .map(|elf| elf.chars().collect::<HashSet<_>>());
        let mut common = elfsets.next().unwrap();
        for elf in elfsets {
            common = common.intersection(&elf).copied().collect();
        }
        if common.len() != 1 { panic!() }
        for c in common { parttwo += value(&c) }
    }
    (partone, parttwo)
}

fn value(c: &char) -> i32 {
    match c.is_uppercase() {
        false => (*c as i32) - ('a' as i32) + 1,
        true => (*c as i32) - ('A' as i32) + 27,
    }
}