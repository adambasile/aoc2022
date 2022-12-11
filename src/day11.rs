use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Action {
    first: Option<i64>,
    operator: Operator,
    second: Option<i64>,
}

#[derive(Debug, Clone)]
struct Test {
    modulo: i64,
    true_dest: i32,
    false_dest: i32,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    action: Action,
    test: Test,
    items_inspected: i64,
}

pub fn day11(lines: Vec<String>) -> (i64, i64) {
    let mut monkeys = parse_monkeys(lines);
    let partone = monkey_business(&mut monkeys.clone(), 20, 3);
    let parttwo = monkey_business(&mut monkeys, 10000, 1);
    (partone, parttwo)
}

fn monkey_business(monkeys: &mut HashMap<i32, Monkey>, rounds: i32, relief: i64) -> i64 {
    let mut monkey_keys: Vec<_> = monkeys.keys().copied().collect();
    let multiple: i64 = monkeys
        .values()
        .map(|monkey| monkey.test.modulo)
        .chain([relief])
        .product::<i64>();
    monkey_keys.sort();
    for _ in 0..rounds {
        for monkey_key in &monkey_keys {
            let mut monkey = monkeys.get_mut(monkey_key).unwrap();
            monkey.items_inspected += monkey.items.len() as i64;
            let modified: Vec<_> = monkey
                .items
                .drain(..)
                .map(|item| {
                    let action = &monkey.action;
                    let first = action.first.unwrap_or(item);
                    let second = action.second.unwrap_or(item);
                    (match action.operator {
                        Operator::Add => first + second,
                        Operator::Mul => first * second,
                    }) / relief
                })
                .map(|item| {
                    let test = &monkey.test;
                    match item.rem_euclid(test.modulo) == 0 {
                        true => (test.true_dest, item.rem_euclid(multiple)),
                        false => (test.false_dest, item.rem_euclid(multiple)),
                    }
                })
                .collect();
            for (dest, item) in modified {
                monkeys.get_mut(&dest).unwrap().items.push(item);
            }
        }
    }
    let mut inspected: Vec<_> = monkeys
        .values()
        .map(|monkey| monkey.items_inspected)
        .collect();
    inspected.sort();
    inspected.iter().rev().take(2).product()
}

fn parse_monkeys(lines: Vec<String>) -> HashMap<i32, Monkey> {
    let monkey_specs: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
    let mut monkeys = HashMap::<i32, Monkey>::new();
    let monkey_re = Regex::new(r"^Monkey (\d+):$").unwrap();
    let items_re = Regex::new(r"^ {2}Starting items: (.+)$").unwrap();
    let operation_re = Regex::new(r"^ {2}Operation: new = (\w+) (.) (.+)$").unwrap();
    let test_re = Regex::new(r"^ {2}Test: divisible by (\d+)$").unwrap();
    let true_re = Regex::new(r"^ {4}If true: throw to monkey (\d+)$").unwrap();
    let false_re = Regex::new(r"^ {4}If false: throw to monkey (\d+)$").unwrap();
    for spec in monkey_specs {
        let mut spec = spec.iter();
        let monkey_num: i32 = get_first_capture(&monkey_re, spec.next().unwrap());
        let items: Vec<i64> = get_first_capture::<String>(&items_re, spec.next().unwrap())
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let operation_line = operation_re.captures(spec.next().unwrap()).unwrap();
        let first: Option<i64> = match operation_line.get(1).unwrap().as_str() {
            "old" => None,
            val => Some(val.parse::<i64>().unwrap()),
        };
        let operator = match operation_line.get(2).unwrap().as_str() {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!(),
        };
        let second: Option<i64> = match operation_line.get(3).unwrap().as_str() {
            "old" => None,
            val => Some(val.parse::<i64>().unwrap()),
        };
        let action = Action {
            first,
            operator,
            second,
        };
        let modulo: i64 = get_first_capture(&test_re, spec.next().unwrap());
        let true_dest: i32 = get_first_capture(&true_re, spec.next().unwrap());
        let false_dest: i32 = get_first_capture(&false_re, spec.next().unwrap());
        let test = Test {
            modulo,
            true_dest,
            false_dest,
        };
        let monkey = Monkey {
            items,
            action,
            test,
            items_inspected: 0,
        };
        monkeys.insert(monkey_num, monkey);
    }
    monkeys
}
fn get_first_capture<T: FromStr>(re: &Regex, s: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    let capture = match re.captures(s) {
        Some(cap) => cap.get(1).unwrap().as_str(),
        None => panic!("{}\n{}", re, s),
    };
    capture.parse().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines_from_file;
    use std::path::PathBuf;
    #[test]
    fn test_day_11() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day11.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day11(lines), (69918, 19573408701));
    }
    #[test]
    fn test_day_11_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day11test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day11(lines), (10605, 2713310158));
    }
}
