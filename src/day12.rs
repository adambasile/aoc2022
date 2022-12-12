use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Heightmap {
    width: usize,
    adjacency_list: Vec<Vec<usize>>,

    start: usize,
    end: usize,
}

impl Heightmap {
    fn pos_to_name(&self, i: usize, j: usize) -> usize {
        i * self.width + j
    }
    #[allow(dead_code)]
    fn name_to_pos(&self, name: usize) -> (usize, usize) {
        (name / self.width, name % self.width)
    }
    fn new(lines: &Vec<String>) -> Heightmap {
        let width = lines.first().unwrap().len();
        // temp so we have access to .pos_to_name()
        let temp = Heightmap {
            width,
            adjacency_list: vec![],
            start: 0,
            end: 0,
        };
        let adjacency_list: Vec<Vec<_>> = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                let lines = &lines;
                let out = &temp;
                line.chars().enumerate().map(move |(j, height)| {
                    let height = char_to_height(height);
                    [
                        Some((i + 1, j)),
                        Some((i, j + 1)),
                        i.checked_sub(1).map(|val| (val, j)),
                        j.checked_sub(1).map(|val| (i, val)),
                    ]
                    .iter()
                    .filter_map(|other| *other)
                    .filter_map(|(m, n)| match lines.get(m) {
                        Some(other_line) => other_line
                            .chars()
                            .nth(n)
                            .map(|c| ((m, n), char_to_height(c))),
                        None => None,
                    })
                    .filter(|(_, h)| (height - h) <= 1)
                    .map(|((posx, posy), _)| out.pos_to_name(posx, posy))
                    .collect()
                })
            })
            .collect();
        let (start, end) = find_start_end(lines, temp);
        Heightmap {
            width,
            adjacency_list,
            start,
            end,
        }
    }
    #[allow(dead_code)]
    fn print_graph(&self) {
        for (i, neighbours) in self.adjacency_list.iter().enumerate() {
            let left = neighbours.contains(&i.wrapping_sub(1));
            let up = neighbours.contains(&i.wrapping_sub(self.width));
            let right = neighbours.contains(&(i + 1));
            let down = neighbours.contains(&(i + self.width));
            let c = match (left, up, right, down) {
                (true, true, true, true) => '┼',
                (true, true, true, false) => '┴',
                (true, true, false, true) => '┤',
                (true, true, false, false) => '┘',
                (true, false, true, true) => '┬',
                (true, false, true, false) => '─',
                (true, false, false, true) => '┐',
                (true, false, false, false) => '╴',
                (false, true, true, true) => '├',
                (false, true, true, false) => '└',
                (false, true, false, true) => '│',
                (false, true, false, false) => '╵',
                (false, false, true, true) => '┌',
                (false, false, true, false) => '╶',
                (false, false, false, true) => '╷',
                (false, false, false, false) => ' ',
            };
            print!("{}", c);
            if (i + 1) % self.width == 0 {
                println!()
            }
        }
    }
}
fn char_to_height(c: char) -> i32 {
    (match c {
        'S' => 'a',
        'E' => 'z',
        d => d,
    }) as i32
}

pub fn day12(lines: Vec<String>) -> (i32, i32) {
    let heightmap = Heightmap::new(&lines);
    let (cost, next) = bfs(&heightmap.adjacency_list, heightmap.end);
    let dayone = dayone(&heightmap, next);
    let daytwo = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let heightmap = &heightmap;
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'a')
                .map(move |(j, _)| (heightmap).pos_to_name(i, j))
        })
        .map(|pos| cost[pos])
        .min()
        .unwrap() as i32;
    (dayone, daytwo)
}

fn dayone(heightmap: &Heightmap, next: Vec<Option<usize>>) -> i32 {
    let mut dayone = 0;
    let mut current = heightmap.start;
    loop {
        current = match next[current] {
            Some(val) => val,
            None => break,
        };
        dayone += 1;
    }
    dayone
}

fn find_start_end(lines: &[String], heightmap: Heightmap) -> (usize, usize) {
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => start = Some(heightmap.pos_to_name(i, j)),
                'E' => end = Some(heightmap.pos_to_name(i, j)),
                _ => {}
            }
        }
    }
    (start.unwrap(), end.unwrap())
}
fn bfs(adjacency_list: &Vec<Vec<usize>>, dest: usize) -> (Vec<i32>, Vec<Option<usize>>) {
    let mut cost: Vec<_> = (0..adjacency_list.len()).map(|_| i32::MAX).collect();
    let mut prev: Vec<Option<usize>> = (0..adjacency_list.len()).map(|_| None).collect();
    let mut queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    cost[dest] = 0;
    queue.push((Reverse(0), dest));
    while let Some((Reverse(base_cost), vertex)) = queue.pop() {
        if base_cost > cost[vertex] {
            continue;
        }
        let alt_cost = base_cost + 1;
        for &neighbour in &adjacency_list[vertex] {
            if alt_cost < cost[neighbour] {
                cost[neighbour] = alt_cost;
                prev[neighbour] = Some(vertex);
                queue.push((Reverse(alt_cost), neighbour))
            }
        }
    }
    (cost, prev)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_12() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day12.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day12(lines), (449, 443));
    }
    #[test]
    fn test_day_12_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day12test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day12(lines), (31, 29));
    }
}
