use std::collections::HashSet;

pub fn day14(lines: Vec<String>) -> (i32, i32) {
    let rocks = parse_rocks(lines);
    let sand_partone = drop_sand(&rocks, (500, 0), false);
    let sand_parttwo = drop_sand(&rocks, (500, 0), true);
    // println!("{}", scan_to_string(&rocks, Some(&sand_parttwo)));
    (sand_partone.len() as i32, sand_parttwo.len() as i32)
}

fn drop_sand(
    rocks: &HashSet<(usize, usize)>,
    sand_start: (usize, usize),
    simulate_floor: bool,
) -> HashSet<(usize, usize)> {
    let floor = 2 + rocks.iter().map(|&(_, y)| y).max().unwrap();
    let mut placed_sand: HashSet<(usize, usize)> = HashSet::new();
    let open = |x, y, placed_sand: &HashSet<(usize, usize)>| {
        y < floor && !(rocks.contains(&(x, y)) || placed_sand.contains(&(x, y)))
    };
    let mut sandstream = vec![sand_start];
    while let Some(pos) = sandstream.pop() {
        let (mut x, mut y) = pos;
        if y + 1 == floor && !simulate_floor {
            break;
        }
        if open(x, y + 1, &placed_sand) {
            y += 1;
        } else if open(x - 1, y + 1, &placed_sand) {
            (x, y) = (x - 1, y + 1);
        } else if open(x + 1, y + 1, &placed_sand) {
            (x, y) = (x + 1, y + 1);
        } else {
            placed_sand.insert(pos);
            continue;
        }
        sandstream.push(pos);
        sandstream.push((x, y))
    }
    placed_sand
}

fn parse_rocks(lines: Vec<String>) -> HashSet<(usize, usize)> {
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    for line in lines {
        let parsed: Vec<_> = line
            .split(" -> ")
            .map(|pair| {
                let pair: Vec<_> = pair
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect();
                (pair[0], pair[1])
            })
            .collect();
        for (&(ix, iy), &(jx, jy)) in parsed.iter().zip(parsed[1..].iter()) {
            let (ix_sort, jx_sort) = if ix <= jx { (ix, jx) } else { (jx, ix) };
            let (iy_sort, jy_sort) = if iy <= jy { (iy, jy) } else { (jy, iy) };
            for x in (ix_sort)..=jx_sort {
                for y in iy_sort..=jy_sort {
                    rocks.insert((x, y));
                }
            }
        }
    }
    rocks
}
#[allow(dead_code)]
fn scan_to_string(
    rocks: &HashSet<(usize, usize)>,
    sand: Option<&HashSet<(usize, usize)>>,
) -> String {
    let no_sand = HashSet::new();
    let sand = sand.unwrap_or(&no_sand);
    let chain = || rocks.iter().chain(sand.iter());
    let ((minx, miny), (maxx, maxy)) = (
        (
            chain().map(|&(x, _)| x).min().unwrap(),
            chain().map(|&(_, y)| y).min().unwrap(),
        ),
        (
            chain().map(|&(x, _)| x).max().unwrap(),
            chain().map(|&(_, y)| y).max().unwrap(),
        ),
    );
    (miny..=maxy)
        .flat_map(|y| {
            (minx..=maxx)
                .map(move |x| {
                    if rocks.contains(&(x, y)) {
                        '#'
                    } else if sand.contains(&(x, y)) {
                        'o'
                    } else {
                        '.'
                    }
                })
                .chain("\n".chars())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines_from_file;
    use std::path::PathBuf;
    #[test]
    fn test_day_14() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day14.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day14(lines), (901, 24589));
    }
    #[test]
    fn test_day_14_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day14test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day14(lines), (24, 93));
    }
}
