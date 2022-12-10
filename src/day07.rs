use std::collections::HashMap;

// Files are owned by directories. Directories are owned by the FileSystem
struct File {
    name: String,
    size: i32,
}

struct Directory {
    path: String, // should include a trailing '/'
    files: Vec<File>,
    directories: Vec<String>,
}

struct FileSystem {
    directories: HashMap<String, Directory>,
}

impl Directory {
    fn new(path: &str) -> Directory {
        Directory {
            path: path.to_string(),
            files: Default::default(),
            directories: Default::default(),
        }
    }

    fn size(&self, filesystem: &FileSystem, store: &mut HashMap<String, i32>) -> i32 {
        let file_sizes = self.files.iter().map(|file| file.size);
        let dir_sizes = self
            .directories
            .iter()
            .map(|dir| filesystem.get(dir).size(filesystem, store));
        let size: i32 = file_sizes.chain(dir_sizes).sum();
        store.insert(self.path.clone(), size);
        size
    }
}

impl FileSystem {
    fn new(lines: Vec<String>) -> FileSystem {
        let mut cwd = String::from("/");
        let root_dir = Directory::new(&cwd);
        let mut filesystem = FileSystem {
            directories: Default::default(),
        };
        filesystem.add(root_dir);
        for line in lines {
            let first_char = line.chars().into_iter().next().unwrap();
            match first_char {
                '$' => {
                    // a command!
                    let mut tokens = line.split_whitespace();
                    tokens.next(); // should be a $
                    let cmd = tokens.next().unwrap();
                    match cmd {
                        "ls" => {} // noop
                        "cd" => change_dir(&mut cwd, tokens.next().unwrap()),
                        _ => unreachable!(),
                    }
                }
                _ => filesystem.process_ls_output(&line, &cwd),
            }
        }
        filesystem
    }

    fn process_ls_output(&mut self, line: &str, cwd: &str) {
        let mut tokens = line.split_whitespace();
        let first_token = tokens.next().unwrap();
        let second_token = tokens.next().unwrap();
        match first_token {
            "dir" => self.add_directory(cwd, second_token),
            size => {
                let size: i32 = size.parse().unwrap();
                self.add_file(cwd, second_token, size)
            }
        }
    }

    fn size(&self, store: &mut HashMap<String, i32>) -> i32 {
        self.get("/").size(self, store)
    }

    fn add(&mut self, dir: Directory) -> Option<Directory> {
        self.directories.insert(dir.path.clone(), dir)
    }

    fn add_directory(&mut self, cwd: &str, name: &str) {
        let mut path = cwd.to_string();
        path.push_str(name);
        path.push('/');
        self.add(Directory::new(path.as_str()));
        self.get_mut(cwd).directories.push(path);
    }

    fn add_file(&mut self, cwd: &str, name: &str, size: i32) {
        self.get_mut(cwd).files.push(File {
            name: name.to_string(),
            size,
        });
    }

    fn get(&self, path: &str) -> &Directory {
        self.directories.get(path).unwrap()
    }

    fn get_mut(&mut self, path: &str) -> &mut Directory {
        self.directories.get_mut(path).unwrap()
    }
}

fn change_dir(cwd: &mut String, dir: &str) {
    match dir {
        "/" => {
            // go root yourself
            cwd.clear();
            cwd.push('/')
        }
        ".." => match cwd.as_str() {
            "/" => {} // already at root, can't go any further up
            _ => {
                // go to the parent
                cwd.pop();
                for _ in 0..((cwd.len() - cwd.rfind('/').unwrap()) - 1) {
                    cwd.pop();
                }
            }
        },
        dir => {
            cwd.push_str(dir);
            cwd.push('/');
        }
    }
}

pub fn day07(lines: Vec<String>) -> (i32, i32) {
    let filesystem = FileSystem::new(lines);
    let mut size_store: HashMap<String, i32> = HashMap::new();
    let used_space = filesystem.size(&mut size_store);
    let partone = size_store.values().filter(|size| size <= &&100000).sum();

    let total_capacity = 70000000;
    let desired_space = 30000000;
    let min_to_delete = used_space - (total_capacity - desired_space);
    let parttwo = *size_store
        .values()
        .filter(|size| size >= &&min_to_delete)
        .min()
        .unwrap();
    (partone, parttwo)
}

impl FileSystem {
    #[allow(dead_code)]
    fn print(&self) {
        let mut keys: Vec<_> = self.directories.keys().collect();
        keys.sort();
        for dir in keys {
            let split: Vec<_> = dir.split('/').collect();
            let idx = split.len() - 2;
            for _ in 0..idx {
                print!(" ")
            }
            let name = split.get(idx).unwrap();
            println!("- {}/ (dir)", name);
            for file in &self.get(dir).files {
                for _ in 0..(idx + 1) {
                    print!(" ")
                }
                println!("- {} (file, size={})", file.name, file.size)
            }
        }
    }
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
        assert_eq!(day07(lines), (2104783, 5883165));
    }

    #[test]
    fn test_day_07_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day07test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day07(lines), (95437, 24933642));
    }
}
