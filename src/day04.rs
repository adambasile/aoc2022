pub fn day04(lines: Vec<String>) -> (i32, i32) {
    let assignments = lines
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|assignment| {
                    assignment.split("-").map(|section| section.parse().unwrap()).collect::<Vec<i32>>()
                }).collect::<Vec<Vec<i32>>>()
        }).collect::<Vec<Vec<Vec<i32>>>>();
    let partone: i32 = (&assignments).into_iter().map(|assgns| {
        let ass1 = &assgns[0];
        let ass2 = &assgns[1];
        match ((ass1[0] <= ass2[0]) && (ass1[1] >= ass2[1])) ||
            ((ass1[0] >= ass2[0]) && (ass1[1] <= ass2[1])) {
            true => 1,
            false => 0
        }
    }).sum();
    let parttwo = (&assignments).into_iter().filter(|assgns| {
        let ass1 = &assgns[0];
        let ass2 = &assgns[1];
        overlaps(ass1[0], ass1[1], ass2[0], ass2[1])
    }).count() as i32;
    (partone, parttwo)
}

fn overlaps(starta: i32, enda: i32, startb: i32, endb: i32) -> bool {
    let last_start = starta.max(startb);
    let first_end = enda.min(endb);
    last_start <= first_end
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_04() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day04.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day04(lines), (518, 909));
    }

    #[test]
    fn test_overlap() {
        assert_eq!(overlaps(5, 7, 7, 9), true);
        assert_eq!(overlaps(2, 8, 3, 7), true);
        assert_eq!(overlaps(6, 6, 4, 6), true);
        assert_eq!(overlaps(2, 6, 4, 8), true);
        assert_eq!(overlaps(2, 4, 6, 8), false);
        assert_eq!(overlaps(2, 3, 4, 5), false);
    }
}