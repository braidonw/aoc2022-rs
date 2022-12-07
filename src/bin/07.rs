use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct DirectoryListing(HashMap<String, usize>);
impl DirectoryListing {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, name: &str, size: usize) {
        if self.0.contains_key(name) {
            let existing = self.0.get(name).unwrap();
            self.0.insert(name.to_string(), existing + size);
        } else {
            self.0.insert(name.to_string(), size);
        }
    }

    fn sum_lt_100000(&self) -> usize {
        let mut total = 0;
        for size in self.0.values() {
            if *size < 100000 {
                total += size;
            }
        }
        total
    }

    // Find the smallest value that is greater than the benchmark
    fn smallest_gt_than(&self, benchmark: usize) -> &usize {
        self.0
            .values()
            .filter(|&x| x > &benchmark)
            .sorted_by(|a, b| a.cmp(b))
            .next()
            .unwrap()
    }

    fn used_space(&self) -> &usize {
        self.0.values().max().unwrap()
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List,
}

impl Command {
    // Return the new directory if the command is cd, otherwise return None
    fn process(&self) -> Option<String> {
        match self {
            Command::ChangeDirectory(dir) => Some(dir.to_string()),
            Command::List => None,
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next(); // Skip the $
        match parts.next() {
            Some("cd") => Ok(Command::ChangeDirectory(parts.next().unwrap().to_string())),
            Some("ls") => Ok(Command::List),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut paths: Vec<String> = Vec::new();
    let mut listing = DirectoryListing::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let command = Command::from_str(line).unwrap();

            if let Some(new_dir) = command.process() {
                if new_dir == ".." {
                    paths.pop();
                } else {
                    paths.push(new_dir);
                }
            }
            continue;
        }

        if line.chars().next().unwrap().is_ascii_digit() {
            // println!("Processing output: {:?}", line);
            let size = parse_size(line);
            paths.iter().fold("".to_string(), |acc, x| {
                let name = acc + "/" + x;
                listing.insert(&name, size);
                name
            });
        }
    }

    let sum = listing.sum_lt_100000();

    Some(sum)
}

fn parse_size(line: &str) -> usize {
    let mut parts = line.split_whitespace();
    parts.next().unwrap().parse().unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let total_space = 70000000;
    let update_space = 30000000;

    let mut paths: Vec<String> = Vec::new();
    let mut listing = DirectoryListing::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let command = Command::from_str(line).unwrap();

            if let Some(new_dir) = command.process() {
                if new_dir == ".." {
                    paths.pop();
                } else {
                    paths.push(new_dir);
                }
            }
            continue;
        }

        if line.chars().next().unwrap().is_ascii_digit() {
            let size = parse_size(line);
            paths.iter().fold("".to_string(), |acc, x| {
                let name = acc + "/" + x;
                listing.insert(&name, size);
                name
            });
        }
    }
    let used_space = listing.used_space();
    let unused_space = total_space - used_space;
    let required_space = update_space - unused_space;

    let smallest = listing.smallest_gt_than(required_space);

    Some(*smallest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
