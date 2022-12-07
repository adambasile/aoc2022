use std::collections::HashMap;
use std::fmt;

// create a tree using the arena pattern to resolve ownership


#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: usize,
    files: Vec<usize>,
    directories: HashMap<str, usize>,
}

#[derive(Debug)]
struct FileSystem {
    root: Directory,
    files: Vec<File>,
    directories: Vec<Directory>,
    cwd: usize,
}

pub fn day07(lines: Vec<String>) -> (i32, i32) {
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
