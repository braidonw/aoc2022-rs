use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    for (idx, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
        dbg!(idx);
        println!("{} {} {} {}", a, b, c, d);
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Some((idx + 4) as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bytes = [0 as usize; 14];
    input.chars().enumerate().map(|mut acc, (idx, c)| {
        acc[-1] = c;
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
