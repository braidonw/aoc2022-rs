use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn parse_paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(newline, parse_path)(input)
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    // separated_list0(tag(" -> "), parse_point).map(|points| Path { points })(input)
    let (rest, points) = separated_list0(tag(" -> "), parse_point)(input)?;
    Ok((rest, Path::new(&points)))
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    let (rest, (x, y)) = separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(i)?;

    Ok((rest, Point { x, y }))
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new(points: &[Point]) -> Self {
        Self {
            points: points.to_vec(),
        }
    }

    // Return the points on the straight lines between points
}

fn points_between(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut points = vec![];
    let x1 = p1.x;
    let y1 = p1.y;
    let x2 = p2.x;
    let y2 = p2.y;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;
    while x != x2 || y != y2 {
        points.push(Point { x, y });
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    let paths = parse_paths(input).unwrap().1;
    dbg!(paths);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
