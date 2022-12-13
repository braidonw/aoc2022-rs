use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, *,
};

fn pairs(i: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(item, newline, item).map(|(left, right)| Pair { left, right }),
    )(i)
}

fn item(i: &str) -> IResult<&str, Item> {
    alt((
        delimited(tag("["), separated_list0(tag(","), item), tag("]")).map(Item::List),
        nom::character::complete::u32.map(Item::Number),
    ))(i)
}

#[derive(Debug, Eq, Clone)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}

impl Item {
    fn divider(i: u32) -> Self {
        Item::List(vec![Item::List(vec![Item::Number(i)])])
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::List(left), Item::List(right)) => left == right,
            (Item::Number(left), Item::Number(right)) => left == right,
            (Item::List(left), Item::Number(right)) => left == &vec![Item::Number(*right)],
            (Item::Number(left), Item::List(right)) => &vec![Item::Number(*left)] == right,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::List(left), Item::List(right)) => left.cmp(right),
            (Item::Number(left), Item::Number(right)) => left.cmp(right),
            (Item::List(left), Item::Number(right)) => left.cmp(&vec![Item::Number(*right)]),
            (Item::Number(left), Item::List(right)) => vec![Item::Number(*left)].cmp(right),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    left: Item,
    right: Item,
}

impl Pair {
    fn compare(&self) -> Ordering {
        self.left.cmp(&self.right)
    }

    fn flatten(&self) -> Vec<Item> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn dividers(i: u32, j: u32) -> Self {
        Self {
            left: Item::divider(i),
            right: Item::divider(j),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, pairs) = pairs(input).unwrap();

    let result = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| match pair.compare() {
            Ordering::Less => Some(i as u32),
            _ => None,
        })
        .map(|i| i + 1)
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut pairs) = pairs(input).unwrap();
    pairs.push(Pair::dividers(2, 6));
    let d1 = Item::divider(2);
    let d2 = Item::divider(6);

    let result: u32 = pairs
        .iter()
        .flat_map(|pair| pair.flatten())
        .sorted()
        .enumerate()
        .filter_map(|(i, item)| {
            if item == d1 || item == d2 {
                Some((i + 1) as u32)
            } else {
                None
            }
        })
        .product();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
