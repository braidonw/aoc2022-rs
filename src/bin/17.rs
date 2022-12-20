use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Rock,
    Air,
}

#[derive(Debug)]
struct Shape {
    rocks: Vec<Vec<Rock>>,
    height: u32,
    length: u32,
}

impl Shape {
    fn new(rocks: Vec<Vec<Rock>>) -> Self {
        let height = rocks.len() as u32;
        let length = rocks[0].len() as u32;
        Self {
            rocks,
            height,
            length,
        }
    }
}

struct Brick {
    x: u32,
    y: u32,
    coords: Vec<Position>,
}

impl TryFrom<&Shape> for Brick {
    type Error = ();

    fn try_from(shape: &Shape) -> Result<Self, Self::Error> {
        let mut coords = Vec::new();
        for (y, row) in shape.rocks.iter().enumerate() {
            for (x, rock) in row.iter().enumerate() {
                if *rock == Rock::Rock {
                    coords.push(Position {
                        x: x as u32,
                        y: y as u32,
                    });
                }
            }
        }
        Ok(Self { x: 0, y: 0, coords })
    }
}

#[derive(Debug)]
struct Field {
    rocks: BTreeMap<Position, Rock>,
}

impl Field {
    fn new() -> Self {
        Self {
            rocks: BTreeMap::new(),
        }
    }

    fn drop_rock(&mut self) {
        self.rocks.insert(Position { x: 12, y: 12 }, Rock::Rock);
    }

    fn height(&self) -> u32 {
        self.rocks.keys().map(|p| p.y).max().unwrap_or(0)
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    let (input, shapes) = separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                complete::char('#').map(|_| Rock::Rock),
                complete::char('.').map(|_| Rock::Air),
            ))),
        )
        .map(Shape::new),
    )(input)?;
    Ok((input, shapes))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut field = Field::new();
    let mut moves = parse_moves(input).unwrap().1.iter().cycle();
    let binding = parse_shapes(ROCKS).unwrap();
    let mut shapes = binding.1.iter().cycle();

    let mut rocks_dropped = 0;

    while rocks_dropped < 2022 {
        let brick: Brick = shapes.next().unwrap().try_into().unwrap();
        rocks_dropped += 1;
        field.drop_rock();
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
