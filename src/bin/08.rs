use std::str::FromStr;

#[derive(Debug)]
struct Visiblity {
    from_left: Vec<Vec<bool>>,
    from_right: Vec<Vec<bool>>,
    from_down: Vec<Vec<bool>>,
    from_up: Vec<Vec<bool>>,
}

impl Visiblity {
    fn new(rows: usize, cols: usize) -> Self {
        Visiblity {
            from_left: vec![vec![false; cols]; rows],
            from_right: vec![vec![false; cols]; rows],
            from_down: vec![vec![false; cols]; rows],
            from_up: vec![vec![false; cols]; rows],
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..self.from_left.len() {
            for j in 0..self.from_left[i].len() {
                if self.from_left[i][j]
                    || self.from_right[i][j]
                    || self.from_down[i][j]
                    || self.from_up[i][j]
                {
                    score += 1;
                }
            }
        }
        score
    }
}

#[derive(Debug)]
struct Grid {
    heights: Vec<Vec<usize>>,
    visibility: Visiblity,
    rows: usize,
    cols: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = Vec::new();
        for line in s.lines() {
            let heights: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            trees.push(heights);
        }
        let rows: usize = trees.iter().len();
        let cols: usize = trees[0].len();

        Ok(Grid {
            heights: trees,
            visibility: Visiblity::new(rows, cols),
            rows,
            cols,
        })
    }
}

impl Grid {
    fn calculate_visibility(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let height = &self.heights[i][j];

                if j == 0 {
                    self.visibility.from_left[i][j] = true;
                }
                if i == 0 {
                    self.visibility.from_up[i][j] = true;
                }
                if i == self.rows - 1 {
                    self.visibility.from_down[i][j] = true;
                }
                if j == self.cols - 1 {
                    self.visibility.from_right[i][j] = true;
                }

                // from left
                for k in 0..j {
                    if &self.heights[i][k] >= height {
                        break;
                    }
                    self.visibility.from_left[i][j] = true;
                }

                // from right
                for k in (j + 1)..self.cols {
                    if self.heights[i][k] >= *height {
                        break;
                    }
                    self.visibility.from_right[i][j] = true;
                }

                // from up
                for k in 0..i {
                    if self.heights[k][j] >= *height {
                        break;
                    }
                    self.visibility.from_up[i][j] = true;
                }

                // from down
                for k in (i + 1)..self.rows {
                    if self.heights[k][j] >= *height {
                        break;
                    }
                    self.visibility.from_down[i][j] = true;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::from_str(input).unwrap();
    grid.calculate_visibility();
    let score = grid.visibility.score();
    dbg!(grid.visibility);
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
