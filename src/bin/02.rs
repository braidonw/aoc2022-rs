use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Shape {
    Scissors,
    Paper,
    Rock,
}

impl TryFrom<char> for Shape {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err("invalid shape".into()),
        }
    }
}
impl Shape {
    const ALL: [Shape; 3] = [Shape::Scissors, Shape::Paper, Shape::Rock];

    fn winning_shape(self) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|s| s.beats(self))
            .expect("at least 1 winning move")
    }

    fn losing_shape(self) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("at least 1 losing move")
    }

    fn drawing_shape(self) -> Self {
        self
    }

    fn beats(self, other: Shape) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_shape(self, other: Shape) -> Shape {
        match self {
            Outcome::Win => other.winning_shape(),
            Outcome::Draw => other.drawing_shape(),
            Outcome::Loss => other.losing_shape(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("invalid outcome".into()),
        }
    }
}

struct Round {
    me: Shape,
    other: Shape,
}

impl Round {
    fn new(me: Shape, other: Shape) -> Self {
        Self { me, other }
    }

    fn evaluate(&self) -> Outcome {
        if self.me.beats(self.other) {
            Outcome::Win
        } else if self.other.beats(self.me) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn score(&self) -> usize {
        self.me.score() + self.evaluate().score()
    }
}

impl FromStr for Round {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 3);
        let mut chars = s.chars();
        let (Some(other), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Invalid round".into());
        };
        let other = Shape::try_from(other)?;
        let outcome = Outcome::try_from(outcome)?;
        let me = outcome.matching_shape(other);
        Ok(Round::new(me, other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score: usize = input
        .lines()
        .map(|line| line.parse::<Round>().unwrap().score())
        .sum();
    Some(total_score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_score: usize = input
        .lines()
        .map(|line| line.parse::<Round>().unwrap().score())
        .sum();
    Some(total_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
