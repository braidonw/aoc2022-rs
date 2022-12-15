use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
    beacon_distance: i32,
}

impl Pair {
    fn new(sensor: Point, beacon: Point) -> Self {
        let beacon_distance = sensor.manhattan_distance_to(&beacon);
        Self {
            sensor,
            beacon,
            beacon_distance,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    let (input, pairs) = separated_list1(newline, parse_pair)(input)?;
    Ok((input, pairs))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, (sensor, beacon)) = separated_pair(parse_sensor, tag(": "), parse_beacon)(input)?;
    Ok((input, Pair::new(sensor, beacon)))
}

fn parse_sensor(i: &str) -> IResult<&str, Point> {
    let (i, (_, x, _, y)) = tuple((
        tag("Sensor at x="),
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
    ))(i)?;
    Ok((i, Point { x, y }))
}

fn parse_beacon(i: &str) -> IResult<&str, Point> {
    let (i, (_, x, _, y)) = tuple((
        tag("closest beacon is at x="),
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
    ))(i)?;
    Ok((i, Point { x, y }))
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
struct Grid {}
pub fn part_one(input: &str, y: i32) -> Option<u32> {
    let (_, pairs) = parse(input).unwrap();
    dbg!(pairs);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input, 2000000);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
