use std::collections::HashSet;

pub fn day08(lines: Vec<String>) -> (i32, i32) {
    let trees = parse_trees(lines);
    let partone = partone(&trees);
    let parttwo = parttwo(&trees);
    (partone, parttwo)
}

fn parse_trees(lines: Vec<String>) -> Vec<Vec<(usize, usize, i32)>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, height)| (i, j, height.to_digit(10).unwrap() as i32))
                .collect()
        }).collect()
}

fn partone(trees: &[Vec<(usize, usize, i32)>]) -> i32 {
    let visible_from_left = trees
        .iter()
        .map(|line_of_trees| get_visible_trees(line_of_trees.iter()));
    let visible_from_right = trees
        .iter()
        .map(|line_of_trees| get_visible_trees(line_of_trees.iter().rev()));
    let width = trees.first().unwrap().len();
    let visible_from_top = (0..width)
        .map(|j| trees.iter().map(move |row| row.get(j).unwrap()))
        .map(get_visible_trees);
    let visible_from_bottom = (0..width)
        .map(|j| trees.iter().map(move |row| row.get(j).unwrap()))
        .map(|line_of_trees| get_visible_trees(line_of_trees.rev()));
    visible_from_left
        .chain(visible_from_right)
        .chain(visible_from_top)
        .chain(visible_from_bottom)
        .flatten()
        .collect::<HashSet<_>>()
        .len() as i32
}

fn get_visible_trees<'a, T>(line_of_trees_iter: T) -> Vec<(usize, usize)>
    where T: Iterator<Item=&'a (usize, usize, i32)>
{
    let mut highest_height = -1;
    line_of_trees_iter
        .filter(|(_, _, tree)| match tree > &highest_height {
            true => {
                highest_height = *tree;
                true
            }
            false => false,
        })
        .map(|(i, j, _)| (*i, *j))
        .collect::<Vec<_>>()
}

fn parttwo(trees: &Vec<Vec<(usize, usize, i32)>>) -> i32 {
    let (height, width) = (trees.len(), trees.first().unwrap().len());
    (0..height)
        .flat_map(|i| {
            (0..width).map(move |j| {
                let (_, _, tree_height) = trees.get(i).unwrap().get(j).unwrap();
                get_lines_of_trees_from_tree(trees, i, j)
                    .map(|tree_line| get_visible_trees_from_tree(tree_height, tree_line))
                    .into_iter()
                    .product()
            })
        })
        .max().unwrap()
}

fn get_lines_of_trees_from_tree(
    trees: &[Vec<(usize, usize, i32)>],
    tree_i: usize,
    tree_j: usize,
) -> [Vec<(usize, usize, i32)>; 4] {
    let trees_row = trees.get(tree_i).unwrap();
    let (left_line, right_line) = trees_row.split_at(tree_j);
    let mut left_line: Vec<_> = left_line.into();
    left_line.reverse();
    let trees_column: Vec<_> = trees.iter().map(|row| *row.get(tree_j).unwrap()).collect();
    let (top_line, bottom_line) = trees_column.split_at(tree_i);
    let mut top_line: Vec<_> = top_line.into();
    top_line.reverse();
    [left_line, right_line[1..].into(), top_line, bottom_line[1..].into()]
}

fn get_visible_trees_from_tree(tree_height: &i32, line_of_trees: Vec<(usize, usize, i32)>) -> i32 {
    for (i, (_, _, height)) in line_of_trees.iter().enumerate() {
        if height >= tree_height { return (i + 1) as i32; }
    }
    line_of_trees.len() as i32
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_08() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day08.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day08(lines), (1843, 180000));
    }

    #[test]
    fn test_day_08_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day08test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day08(lines), (21, 8));
    }
}
