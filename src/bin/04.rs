use std::str::FromStr;

#[derive(Debug)]
struct Range {
    lower: usize,
    upper: usize,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let lower = parts.next().unwrap().parse().unwrap();
        let upper = parts.next().unwrap().parse().unwrap();
        Ok(Range { lower, upper })
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn contains_or_is_contained_by(&self, other: &Range) -> bool {
        self.contains(other) || other.contains(self)
    }

    fn overlaps_with(self, other: &Range) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut ranges = line.split(',');
        let (a, b) = (
            ranges.next().unwrap().parse::<Range>().unwrap(),
            ranges.next().unwrap().parse::<Range>().unwrap(),
        );
        if a.contains_or_is_contained_by(&b) {
            count += 1;
        };
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let mut ranges = line.split(',');
        let (a, b) = (
            ranges.next().unwrap().parse::<Range>().unwrap(),
            ranges.next().unwrap().parse::<Range>().unwrap(),
        );
        if a.overlaps_with(&b) {
            count += 1;
        };
    }
    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
