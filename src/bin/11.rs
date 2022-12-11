use std::{cell::RefCell, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: u128,
}

impl Item {
    fn apply(&mut self, operation: &Operation) {
        self.worry_level = operation.apply(self.worry_level);
    }

    fn div(&mut self, n: u128) {
        self.worry_level /= n;
    }

    fn modulo(&mut self, n: u128) {
        self.worry_level %= n;
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let worry_level = s.parse().expect(s);
        Ok(Item { worry_level })
    }
}

#[derive(Debug)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(23) == Some('+') {
            Ok(Operation::Add(s[25..].parse().unwrap()))
        } else if &s[25..] == "old" {
            Ok(Operation::Square)
        } else if s.chars().nth(23) == Some('*') {
            Ok(Operation::Multiply(s[25..].parse().unwrap()))
        } else {
            panic!("Unknown operation: {}", s)
        }
    }
}

impl Operation {
    fn apply(&self, x: u128) -> u128 {
        match self {
            Operation::Add(n) => x + n,
            Operation::Multiply(n) => x * n,
            Operation::Square => x * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<Item>>,
    operation: Operation,
    test_divisor: u128,
    target_true: usize,
    target_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _name = lines.next();
        let items = RefCell::new(
            lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split(", ")
                .map(|c| c.trim().parse().unwrap())
                .collect(),
        );

        let operation = lines.next().unwrap().parse().unwrap();
        let test_divisor = lines.next().unwrap()[21..].parse().unwrap();
        let target_true = lines.next().unwrap()[29..].parse().unwrap();
        let target_false = lines.next().unwrap()[30..].parse().unwrap();
        Ok(Monkey {
            items,
            operation,
            test_divisor,
            target_true,
            target_false,
        })
    }
}

impl Monkey {
    fn test(&self, item: &Item) -> bool {
        item.worry_level % self.test_divisor == 0
    }

    fn print_items(&self) {
        println!("Monkey: {:?}", self.items.borrow());
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut inspect_counts = vec![0; monkeys.len()];

    for i in 1..=20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.borrow_mut().iter_mut() {
                inspect_counts[i] += 1;
                item.apply(&monkey.operation);
                item.div(3);
                let target = if monkey.test(item) {
                    monkey.target_true
                } else {
                    monkey.target_false
                };

                monkeys[target].items.borrow_mut().push(*item);
            }
            monkey.items.borrow_mut().clear();
        }

        // println!("Round: {}", i);
        // for monkey in &monkeys {
        //     monkey.print_items();
        // }
        // println!();
    }

    let monkey_business: u128 = inspect_counts
        .iter()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product();

    Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<u128> {
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut inspect_counts = vec![0; monkeys.len()];

    let lcm: u128 = monkeys.iter().map(|m| m.test_divisor).product();
    for _ in 1..=10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.borrow_mut().iter_mut() {
                inspect_counts[i] += 1;
                item.apply(&monkey.operation);
                item.modulo(lcm);
                let target = if monkey.test(item) {
                    monkey.target_true
                } else {
                    monkey.target_false
                };

                monkeys[target].items.borrow_mut().push(*item);
            }
            monkey.items.borrow_mut().clear();
        }

        // println!("Round: {}", i);
        // for monkey in &monkeys {
        //     monkey.print_items();
        // }
        // println!();
    }

    let monkey_business: u128 = inspect_counts
        .iter()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product();

    Some(monkey_business)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
