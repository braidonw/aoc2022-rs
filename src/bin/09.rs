use std::{collections::HashSet, str::FromStr};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}
impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
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

    fn touches(&self, other: &Knot) -> bool {
        let x_distance = (self.x - other.x).abs();
        let y_distance = (self.y - other.y).abs();

        x_distance < 2 && y_distance < 2
    }

    fn move_relative_to(&mut self, other: &Knot) {
        if self.x == other.x {
            if self.y < other.y {
                self.step(&Direction::Up)
            } else {
                self.step(&Direction::Down)
            }
        } else if self.y == other.y {
            if self.x < other.x {
                self.step(&Direction::Right)
            } else {
                self.step(&Direction::Left)
            }
        } else {
            if self.y < other.y {
                self.step(&Direction::Up)
            } else {
                self.step(&Direction::Down)
            }
            if self.x < other.x {
                self.step(&Direction::Right)
            } else {
                self.step(&Direction::Left)
            }
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
    tail_positions: HashSet<Knot>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let knots = vec![Knot::new(); size];
        let mut positions = HashSet::new();
        positions.insert(Knot::new());
        Rope {
            knots,
            tail_positions: positions,
        }
    }

    fn handle_command(&mut self, command: Command) {
        for _ in 0..command.distance {
            self.move_rope(&command.direction);
            self.tail_positions.insert(*self.knots.last().unwrap());
        }
    }

    fn move_rope(&mut self, direction: &Direction) {
        self.knots.iter_mut().fold(None, |prev, knot| {
            // if the first iteration, set prev to knot and continue
            if prev.is_none() {
                knot.step(direction);
                return Some(*knot);
            }

            // if the knots touch, no movement necessary
            if knot.touches(&prev.unwrap()) {
                return Some(*knot);
            }

            // If the knot doesn't touch the previous, then move it relative to the previous
            knot.move_relative_to(&prev.unwrap());
            Some(*knot)
        });
    }

    fn print(&self) {
        for (i, knot) in self.knots.iter().enumerate() {
            println!("Knot {} - {}, {}", i, knot.x, knot.y);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rope = Rope::new(2);
    let commands: Vec<_> = input
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();
    for command in commands {
        rope.handle_command(command);
    }
    Some(rope.tail_positions.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rope = Rope::new(10);
    let commands: Vec<_> = input
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();

    for command in commands {
        rope.handle_command(command);
    }
    rope.print();
    Some(rope.tail_positions.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};

    fn read_file(folder: &str, name: &str) -> String {
        let cwd = env::current_dir().unwrap();

        let filepath = cwd.join("src").join(folder).join(format!("{}.txt", name));

        let f = fs::read_to_string(filepath);
        f.expect("could not open input file")
    }

    #[test]
    fn test_part_one() {
        let input = read_file("examples", "09_01");
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", "09_02");
        assert_eq!(part_two(&input), Some(36));
    }
}
