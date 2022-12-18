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

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Debug)]
enum Rock {
    Rock,
    Air,
}
#[derive(Debug)]
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
    let (input, shapes) = separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                complete::char('#').map(|_| Rock::Rock),
                complete::char('.').map(|_| Rock::Air),
            ))),
        )
        .map(|rocks| Shape { rocks }),
    )(input)?;
    Ok((input, shapes))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, moves) = parse_moves(input).unwrap();
    let (_, rocks) = parse_shapes(ROCKS).unwrap();
    dbg!(&rocks);
    dbg!(&moves);
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
