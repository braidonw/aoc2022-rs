use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
struct Height(u32);
impl TryFrom<char> for Height {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Height(1)),
            'E' => Ok(Height(26)),
            c => Ok(Height(c as u32 - 'a' as u32 + 1)),
        }
    }
}

impl Height {
    fn can_reach(&self, other: &Self) -> bool {
        self.0 + 1 >= other.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(usize, usize);
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position(x, y)
    }
}

#[derive(Debug)]
struct Grid {
    start: Position,
    end: Position,
    rows: usize,
    cols: usize,
    heights: Vec<Vec<Height>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();
        let mut start = Position::new(0, 0);
        let mut end = Position::new(0, 0);
        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                row.push(Height::try_from(c)?);
                if c == 'S' {
                    start = Position::new(i, j);
                }

                if c == 'E' {
                    end = Position::new(i, j);
                }
            }
            heights.push(row);
        }

        Ok(Grid {
            rows: heights.len(),
            cols: heights[0].len(),
            heights,
            start,
            end,
        })
    }
}

impl Grid {
    fn height_for(&self, pos: &Position) -> &Height {
        &self.heights[pos.0][pos.1]
    }

    fn neighbours_for(&self, pos: &Position) -> Vec<Position> {
        let mut neighbours = Vec::new();
        if pos.0 > 0 {
            neighbours.push(Position::new(pos.0 - 1, pos.1));
        }

        if pos.0 < self.rows - 1 {
            neighbours.push(Position::new(pos.0 + 1, pos.1));
        }

        if pos.1 > 0 {
            neighbours.push(Position::new(pos.0, pos.1 - 1));
        }

        if pos.1 < self.cols - 1 {
            neighbours.push(Position::new(pos.0, pos.1 + 1));
        }

        neighbours
    }

    fn bfs(&self, start: Position) -> u32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut best: u32 = 1000000;

        queue.push_back((start, 0));

        while let Some((pos, dist)) = queue.pop_front() {
            if pos == self.end {
                if dist < best {
                    best = dist;
                }
                continue;
            }

            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            for neighbour in self.neighbours_for(&pos) {
                if self.height_for(&pos).can_reach(self.height_for(&neighbour)) {
                    queue.push_back((neighbour, dist + 1));
                }
            }
        }

        best
    }

    fn all_starting_nodes(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.heights[row][col].0 == 1 {
                    positions.push(Position::new(col, row));
                }
            }
        }

        positions
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let path_length = grid.bfs(grid.start);
    Some(path_length as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut distances: Vec<u32> = Vec::new();
    let grid = Grid::from_str(input).unwrap();
    let starts = grid.all_starting_nodes();
    dbg!(&starts);
    // for s in starts {
    //     let dist = grid.bfs(s);
    //     dbg!(s, &dist);
    //     distances.push(dist);
    // }
    distances.sort();
    let dist = grid.bfs(Position::new(0, 4));

    Some(dist)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
