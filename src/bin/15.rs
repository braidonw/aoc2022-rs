use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
#[derive(Debug)]
struct Grid {
    sensors: Vec<Point>,
    beacons: BTreeSet<Point>,
    pairs: BTreeMap<Point, Point>,
}

impl Grid {
    fn new(pairs: Vec<Pair>) -> Self {
        let sensors = pairs.iter().map(|p| p.sensor).collect::<Vec<_>>();
        let beacons: BTreeSet<Point> = pairs.iter().map(|p| p.beacon).collect();

        let sensor_beacon_pairs = pairs
            .iter()
            .map(|p| (p.sensor, p.beacon))
            .collect::<BTreeMap<_, _>>();

        Self {
            sensors,
            beacons,
            pairs: sensor_beacon_pairs,
        }
    }

    fn find_intersections_at_y(&self, line: i32) -> i32 {
        let intersections: BTreeSet<_> = self
            .sensors
            .iter()
            .filter_map(|s| {
                let beacon = self.pairs.get(s).unwrap();
                let distance_to_beacon = s.manhattan_distance_to(beacon);
                let distance_to_line = (s.y - line).abs();
                if distance_to_line > distance_to_beacon {
                    return None;
                }
                let dx = distance_to_beacon - distance_to_line;
                let points: Vec<_> = ((s.x - dx)..=(s.x + dx))
                    .into_iter()
                    .map(|x| Point { x, y: line })
                    .collect();

                Some(points)
            })
            .flatten()
            .collect();

        intersections.iter().len() as i32
            - self.beacons.iter().filter(|b| b.y == line).count() as i32
    }

    fn greated_y_intercept(&self, sensor: &Point, y: i32) -> Option<Point> {
        let beacon = self.pairs.get(sensor).unwrap();
        let distance_to_beacon = sensor.manhattan_distance_to(beacon);
        let distance_to_line = (sensor.y - y).abs();
        if distance_to_line > distance_to_beacon {
            return None;
        }
        let dx = distance_to_beacon - distance_to_line;
        Some(Point {
            x: sensor.x + dx,
            y,
        })
    }

    fn find_distress_beacon_xy(&self, grid_size: u32) -> Point {
        for y in 0..grid_size {
            let mut x: u32 = 0;
            'x: while x <= grid_size {
                for s in &self.sensors {
                    let beacon = self.pairs.get(s).unwrap();

                    let distance_to_beacon = s.manhattan_distance_to(beacon);
                    let distance_to_xy = s.manhattan_distance_to(&Point {
                        x: x as i32,
                        y: y as i32,
                    });

                    dbg!(&s, &beacon, distance_to_beacon, distance_to_xy);

                    if distance_to_xy < distance_to_beacon {
                        let new_point = self.greated_y_intercept(s, y as i32).unwrap();
                        x = new_point.x as u32;
                        continue 'x;
                    }
                }
                println!("No sensor found for {}, {}", x, y);
                return Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
        unreachable!()
    }
}

fn tuning_frequency(point: &Point) -> u32 {
    (point.x * 4000000 + point.y) as u32
}
pub fn part_one(input: &str) -> Option<i32> {
    let (_, pairs) = parse(input).unwrap();
    let grid = Grid::new(pairs);
    let count = grid.find_intersections_at_y(2000000);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, pairs) = parse(input).unwrap();
    let grid = Grid::new(pairs);
    let beacon_point = grid.find_distress_beacon_xy(20);
    let f = tuning_frequency(&beacon_point);

    dbg!(&beacon_point, f);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        let grid = Grid::new(parse(&input).unwrap().1);
        let count = grid.find_intersections_at_y(10);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
