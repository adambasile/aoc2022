use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y
    }
    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y
    }
}

pub fn day09(lines: Vec<String>) -> (i32, i32) {
    let partone = visited_by_tail(&lines, 2);
    let parttwo = visited_by_tail(&lines, 10);
    (partone, parttwo)
}

fn visited_by_tail(lines: &Vec<String>, rope_length: i32) -> i32 {
    let mut rope: Vec<Point> = (0..rope_length).map(|_| Point::new(0, 0)).collect();
    let mut visited: HashSet<Point> = HashSet::from([Point::new(0, 0)]);
    for line in lines {
        let mut split = line.split_whitespace();
        let dirn = split.next().unwrap();
        let steps: i32 = split.next().unwrap().parse().unwrap();
        let delta_head: Point = match dirn {
            "R" => Point::new(1, 0),
            "U" => Point::new(0, 1),
            "L" => Point::new(-1, 0),
            "D" => Point::new(0, -1),
            _ => unreachable!(),
        };
        for _ in 0..steps {
            rope.first_mut().unwrap().add(&delta_head);
            let mut rope_iter = rope.iter_mut();
            let mut previous = rope_iter.next().unwrap();
            for current in rope_iter {
                shorten_rope(previous, current); // this mutates current
                previous = current;
            }
            visited.insert(rope.last().unwrap().clone());
        }
    }
    visited.len() as i32
}

fn shorten_rope(head: &Point, tail: &mut Point) {
    let diff = Point::new(head.x - tail.x, head.y - tail.y);
    if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
        return;
    } else if diff.x.abs() == diff.y.abs() {
        tail.set(head.x - diff.x.signum(), head.y - diff.y.signum());
    } else if diff.x.abs() > diff.y.abs() {
        tail.set(head.x - diff.x.signum(), head.y)
    } else if diff.x.abs() < diff.y.abs() {
        tail.set(head.x, head.y - diff.y.signum())
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_09() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day09.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day09(lines), (5858, 2602));
    }

    #[test]
    fn test_day_09_small1() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day09test1.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day09(lines), (13, 1));
    }

    #[test]
    fn test_day_09_small2() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day09test2.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day09(lines), (88, 36));
    }
}
