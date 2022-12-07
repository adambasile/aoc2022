pub(crate) fn day07(lines: Vec<String>) -> (i32, i32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_07() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day07.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day07(lines), (-1, -1));
    }

    #[test]
    fn test_day_07_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day07test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day07(lines), (95437, -1));
    }
}
