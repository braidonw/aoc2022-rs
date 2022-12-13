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

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}
impl Item {}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, pairs) = pairs(input).unwrap();
    dbg!(&pairs);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
