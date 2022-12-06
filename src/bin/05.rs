use std::str::FromStr;

#[derive(Debug)]
struct Crate(char);
#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: Vec::new() }
    }

    fn push(&mut self, c: Crate) {
        self.crates.push(c);
    }

    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }
}

fn build_stacks(input: &str) -> Vec<Stack> {
    let mut iter = input.split("\n\n").next().unwrap().lines().rev();

    let num_stacks: usize = iter.next().unwrap().trim().replace(' ', "").len();
    let mut stacks: Vec<Stack> = (0..num_stacks).map(|_| Stack::new()).collect();

    for line in iter {
        for (idx, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let idx = idx / 4;
                stacks[idx].push(Crate(c));
            }
        }
    }
    stacks
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next(); // "move"
        let count = parts.next().unwrap().parse().unwrap();
        parts.next(); // "from"
        let from: usize = parts.next().unwrap().parse().unwrap();
        parts.next(); // "to"
        let to: usize = parts.next().unwrap().parse().unwrap();

        Ok(Move {
            from: from - 1,
            to: to - 1,
            count,
        })
    }
}

impl Move {
    fn execute(self, stacks: &mut [Stack]) {
        for _ in 0..self.count {
            let c = stacks[self.from].pop().unwrap();
            stacks[self.to].push(c);
        }
    }

    fn execute_bulk(self, stacks: &mut [Stack]) {
        let mut crates = Vec::new();
        for _ in 0..self.count {
            let c = stacks[self.from].pop().unwrap();
            crates.push(c);
        }
        crates.reverse();
        for c in crates {
            stacks[self.to].push(c);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = build_stacks(input);

    let moves: Vec<Move> = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    for m in moves {
        m.execute(&mut stacks);
    }

    let mut tops = String::new();
    for mut s in stacks {
        let top = s.pop().unwrap();
        tops.push(top.0);
    }
    Some(tops)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks = build_stacks(input);

    let moves: Vec<Move> = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    for m in moves {
        m.execute_bulk(&mut stacks);
    }

    let mut tops = String::new();
    for mut s in stacks {
        let top = s.pop().unwrap();
        tops.push(top.0);
    }
    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
