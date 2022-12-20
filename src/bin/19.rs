#![allow(dead_code)]
use nom::IResult;
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    todo!()
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    todo!()
}

const TIME_LIMIT: usize = 24;

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Factory {
    blueprint: Blueprint,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Cost {
    fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, blueprints) = parse(input).unwrap();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
