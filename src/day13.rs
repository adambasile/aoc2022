use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;
#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}
impl Packet {
    fn new(line: &mut Peekable<Chars>) -> Packet {
        if line.next().unwrap() != '[' {
            unreachable!()
        }
        let mut list: Vec<Packet> = Vec::new();
        let mut number = String::new();
        while let Some(c) = line.peek() {
            match c {
                ',' => Self::push_number(&mut list, &mut number),
                ']' => {
                    Self::push_number(&mut list, &mut number);
                    return Packet::List(list);
                }
                '[' => list.push(Packet::new(line)),
                digit if c.is_ascii_digit() => number.push(*digit),
                _ => unreachable!(),
            }
            line.next();
        }
        unreachable!()
    }
    fn push_number(list: &mut Vec<Packet>, number: &mut String) {
        if !number.is_empty() {
            list.push(Packet::Integer(number.parse().unwrap()));
            number.clear();
        }
    }
}
impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Packet {}
impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self;
        let right = other;
        match (left, right) {
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    match left.cmp(right) {
                        Ordering::Equal => {}
                        val => return val,
                    }
                }
                left.len().cmp(&right.len())
            }
            (Packet::List(_), Packet::Integer(_)) => left.cmp(&Packet::List(vec![right.clone()])),
            (Packet::Integer(_), Packet::List(_)) => Packet::List(vec![left.clone()]).cmp(right),
        }
    }
}
pub fn day13(lines: Vec<String>) -> (i32, i32) {
    let partone = (lines)
        .chunks(3)
        .enumerate()
        .filter(|(_, group)| {
            let left = Packet::new(&mut group[0].chars().peekable());
            let right = Packet::new(&mut group[1].chars().peekable());
            left < right
        })
        .map(|(i, _)| (i + 1) as i32)
        .sum::<i32>();
    let mut sorted_packets: Vec<_> = (lines)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::new(&mut line.chars().peekable()))
        .collect();
    let decoy1 = Packet::new(&mut "[[2]]".to_string().chars().peekable());
    let decoy2 = Packet::new(&mut "[[6]]".to_string().chars().peekable());
    sorted_packets.push(decoy1.clone());
    sorted_packets.push(decoy2.clone());
    sorted_packets.sort();
    let parttwo = sorted_packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| packet == &&decoy1 || packet == &&decoy2)
        .map(|(i, _)| (i + 1) as i32)
        .product::<i32>();
    (partone, parttwo)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines_from_file;
    use std::path::PathBuf;
    #[test]
    fn test_day_13() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day13.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day13(lines), (5684, 22932));
    }
    #[test]
    fn test_day_13_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day13test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day13(lines), (13, 140));
    }
}
