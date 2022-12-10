#[derive(Debug)]
struct State {
    clock: i32,
    register: i32,
}

pub fn day10(lines: Vec<String>) -> (i32, String) {
    let mut clock: i32 = 0;
    let mut register: i32 = 1;
    let mut states: Vec<State> = Vec::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let (cycles, value) = match split.next().unwrap() {
            "noop" => (1, 0),
            "addx" => (2, split.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        };
        for _ in 0..cycles {
            clock += 1;
            states.push(State { clock, register });
        }
        register += value
    }
    let partone = states
        .iter()
        .filter(|state| (state.clock + 20).rem_euclid(40) == 0)
        .map(|state| state.clock * state.register)
        .sum();
    let mut parttwo = "".to_string();
    for state in states.iter() {
        let pixel = (state.clock - 1).rem_euclid(40) + 1;
        if ((state.register - pixel) + 1).abs() <= 1 {
            parttwo.push('#')
        } else {
            parttwo.push('.')
        }
        if state.clock.rem_euclid(40) == 0 {
            parttwo.push('\n')
        }
    }
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_10() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day10.txt");
        let lines = read_lines_from_file(filename);
        let expected_parttwo = "\
###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.
"
        .to_string();
        assert_eq!(day10(lines), (15360, expected_parttwo));
    }

    #[test]
    fn test_day_10_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day10test.txt");
        let lines = read_lines_from_file(filename);
        let expected_parttwo = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .to_string();
        assert_eq!(day10(lines), (13140, expected_parttwo));
    }
}
