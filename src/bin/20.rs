#![warn(dead_code)]

use std::{ops::Rem, str::FromStr};

#[derive(Debug)]
struct File {
    original: Vec<i64>,
    mixed: Vec<i64>,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mixed = original.clone();
        Ok(File { original, mixed })
    }
}

impl File {
    fn mix_val(&mut self, val: &i64) {
        let val_idx = self.mixed.iter().position(|&x| x == *val).unwrap();
        let current = self.mixed.remove(val_idx);
        let mut new_idx = val_idx as i64 + current;
        new_idx = new_idx.rem_euclid(self.mixed.len() as i64);
        if new_idx == 0 {
            new_idx = self.mixed.len() as i64;
        }
        self.mixed.insert(new_idx as usize, current);
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut file = input.parse::<File>().unwrap();
    for val in &file.original.clone() {
        file.mix_val(val);
    }

    let zero_pos = file.mixed.iter().position(|&x| x == 0).unwrap();
    dbg!(zero_pos);
    let a = file.mixed[(1000 + zero_pos) % file.mixed.len()];
    let b = file.mixed[(2000 + zero_pos) % file.mixed.len()];
    let c = file.mixed[(3000 + zero_pos) % file.mixed.len()];

    dbg!(a, b, c);

    Some(a + b + c);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
