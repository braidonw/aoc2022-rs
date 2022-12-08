use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    heights: Vec<Vec<usize>>,
    visible: Vec<Vec<bool>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = Vec::new();
        for line in s.lines() {
            let heights: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            trees.push(heights);
        }
        Ok(Grid {
            heights: trees,
            visible: vec![vec![]],
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    for (i, row) in grid.heights.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            dbg!(i, j, height);
        }
    }
    dbg!(grid);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
