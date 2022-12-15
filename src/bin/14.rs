use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) = separated_list1(newline, parse_line)(input)?;
    let rocks: BTreeSet<(u32, u32)> = pairs.into_iter().flatten().collect();
    Ok((input, rocks))
}

fn parse_line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, char(','), complete::u32),
    )(input)?;

    let iter = pairs
        .into_iter()
        .tuple_windows()
        .flat_map(|((x1, y1), (x2, y2))| {
            let x_range = x1.min(x2)..=x1.max(x2);
            let y_range = y1.min(y2)..=y1.max(y2);

            x_range.cartesian_product(y_range)
        });

    Ok((input, iter))
}

#[derive(Debug)]
struct Grid {
    rocks: BTreeSet<(u32, u32)>,
    num_rocks: u32,
    floor: (u32, u32),
}

impl Grid {
    fn new(rocks: BTreeSet<(u32, u32)>) -> Self {
        let num_rocks = rocks.len() as u32;
        let mut rocks_2 = rocks.iter().collect::<Vec<&(u32, u32)>>();
        rocks_2.sort_by(|a, b| a.1.cmp(&b.1));
        let floor = **rocks_2.last().unwrap();
        Self {
            rocks,
            num_rocks,
            floor,
        }
    }

    fn count_sand(&self) -> u32 {
        (self.rocks.len() - self.num_rocks as usize) as u32
    }

    fn drop_sand(&mut self) {
        let mut sand = (500, 0);
        loop {
            if sand.1 > self.floor.1 {
                break;
            }

            let down = (sand.0, sand.1 + 1);
            let down_left = (sand.0 - 1, sand.1 + 1);
            let down_right = (sand.0 + 1, sand.1 + 1);

            match (
                self.rocks.contains(&down),
                self.rocks.contains(&down_left),
                self.rocks.contains(&down_right),
            ) {
                (true, true, true) => {
                    self.rocks.insert(sand);
                    sand = (500, 0);
                }
                (false, _, _) => {
                    sand = down;
                }
                (_, false, _) => {
                    sand = down_left;
                }
                (_, _, false) => {
                    sand = down_right;
                }
            }
        }
    }

    fn drop_sand_until_at_roof(&mut self) {
        let mut sand = (500, 0);
        while self.rocks.get(&(500, 0)).is_none() {
            let down = (sand.0, sand.1 + 1);
            let down_left = (sand.0 - 1, sand.1 + 1);
            let down_right = (sand.0 + 1, sand.1 + 1);

            match (
                self.rocks.get(&down).or({
                    if down.1 == self.floor.1 + 2 {
                        Some(&self.floor)
                    } else {
                        None
                    }
                }),
                self.rocks.get(&down_left).or({
                    if down_left.1 == self.floor.1 + 2 {
                        Some(&self.floor)
                    } else {
                        None
                    }
                }),
                self.rocks.get(&down_right).or({
                    if down_right.1 == self.floor.1 + 2 {
                        Some(&self.floor)
                    } else {
                        None
                    }
                }),
            ) {
                (Some(_), Some(_), Some(_)) => {
                    self.rocks.insert(sand);
                    sand = (500, 0);
                }
                (None, _, _) => {
                    sand = down;
                }
                (_, None, _) => {
                    sand = down_left;
                }
                (_, _, None) => {
                    sand = down_right;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rocks = parse_rocks(input).ok()?.1;
    let mut grid = Grid::new(rocks);
    grid.drop_sand();
    Some(grid.count_sand())
}

pub fn part_two(input: &str) -> Option<u32> {
    let rocks = parse_rocks(input).ok()?.1;
    let mut grid = Grid::new(rocks);
    grid.drop_sand_until_at_roof();
    Some(grid.count_sand())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
