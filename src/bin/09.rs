use std::str::FromStr;

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: u32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = parts.next().unwrap().parse().unwrap();
        Ok(Command {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }

    fn touches(&self, other: &Position) -> bool {
        let x_distance = (self.x - other.x).abs();
        let y_distance = (self.y - other.y).abs();

        x_distance < 2 && y_distance < 2
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Position::new(),
            tail: Position::new(),
        }
    }

    fn handle_command(&mut self, command: Command) {
        for _ in 0..command.distance {
            self.move_head(&command.direction);

            if self.tail.touches(&self.head) {
                continue;
            }

            self.tail.step(&command.direction);
            if self.tail.touches(&self.head) {
                continue;
            }

            if self.tail.x - self.head.x > 1 {
                self.move_tail(&Direction::Left)
            } else if self.head.x - self.tail.x > 1 {
                self.move_tail(&Direction::Right)
            } else if self.tail.y - self.head.y > 1 {
                self.move_tail(&Direction::Up)
            } else if self.head.y - self.tail.y > 1 {
                self.move_tail(&Direction::Down)
            }
            dbg!(&self);
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.head.step(direction);
    }

    fn move_tail(&mut self, direction: &Direction) {
        self.tail.step(direction);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new();
    let commands: Vec<_> = input
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();
    for command in commands {
        dbg!(&command);
        rope.handle_command(command);
        dbg!(&rope);
    }
    dbg!(rope);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
