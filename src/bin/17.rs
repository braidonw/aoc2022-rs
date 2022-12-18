use nom::{
    branch::alt,
    character::complete::{self, newline},
    complete::tag,
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

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

enum Rock {
    Rock,
    Air,
}

struct Shape {
    rocks: Vec<Vec<Rock>>,
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    let (input, rocks) = separated_list1(
        tag("\n\n"),
        separated_list1(
            newline,
            many1(alt((
                complete::char('.').map(|_| Rock::Air),
                complete::char('#').map(|_| Rock::Rock),
            ))),
        ),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    dbg!(parse_moves(input));
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
