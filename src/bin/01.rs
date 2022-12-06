pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut current = 0;

    for line in input.lines() {
        if let Ok(v) = line.parse::<u32>() {
            current += v;
            if current > max {
                max = current;
            }
        } else {
            current = 0;
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals: Vec<u64> = Vec::new();

    let mut current = 0;

    for line in input.lines() {
        if let Ok(v) = line.parse::<u32>() {
            current += v;
        } else {
            totals.push(current as u64);
            current = 0;
        }
    }
    totals.sort_unstable();
    totals.reverse();
    totals.truncate(3);
    let total = totals.iter().sum::<u64>();
    Some(total as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
