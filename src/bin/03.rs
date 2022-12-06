#[derive(Debug)]
struct Item(usize);

impl TryFrom<&u8> for Item {
    type Error = ();

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        let value = if *value > b'a' {
            *value as u8 - b'a' + 1
        } else {
            *value as u8 - b'A' + 27
        };
        Ok(Self(value as usize))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0usize;

    for line in input.lines() {
        let (p1, p2) = line.as_bytes().split_at(line.len() / 2);

        let mut occurences: [bool; 53] = [false; 53];
        for x in p1 {
            let item = Item::try_from(x).unwrap();
            occurences[item.0] = true;
        }

        for x in p2 {
            let item = Item::try_from(x).unwrap();
            if occurences[item.0] {
                sum += item.0;
                break;
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let score: u32 = lines.chunks(3).map(check_group).sum();
    Some(score)
}

fn check_group(chunk: &[&str]) -> u32 {
    let mut occurences: [u8; 53] = [0; 53];
    let mut sum = 0;

    for (idx, sack) in chunk.iter().enumerate() {
        for element in sack.as_bytes() {
            let item = Item::try_from(element).unwrap();

            occurences[item.0] |= 1 << idx;
            if occurences[item.0] == 7 {
                sum += item.0;
                break;
            }
        }
    }

    sum as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
