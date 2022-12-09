use std::str::FromStr;

#[derive(Debug)]
struct Visiblity(Vec<Vec<bool>>);

impl Visiblity {
    fn new(rows: usize, cols: usize) -> Self {
        let mut v = Vec::new();
        for _ in 0..rows {
            v.push(vec![false; cols]);
        }
        Visiblity(v)
    }

    // return the number of visible trees
    fn score(&self) -> usize {
        let mut score = 0;
        for row in &self.0 {
            for visible in row {
                if *visible {
                    score += 1;
                }
            }
        }
        score
    }

    fn make_visible(&mut self, i: usize, j: usize) {
        self.0[i][j] = true;
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
        // from left
        for i in 0..self.rows {
            let mut max = 0;
            for j in 0..self.cols {
                if j == 0 || self.heights[i][j] > max {
                    max = self.heights[i][j];
                    self.visibility.make_visible(i, j);
                }
            }
        }

        // from up
        for j in 0..self.cols {
            let mut max = 0;
            for i in 0..self.rows {
                if i == 0 || self.heights[i][j] > max {
                    max = self.heights[i][j];
                    self.visibility.make_visible(i, j);
                }
            }
        }

        // from right
        for i in 0..self.rows {
            let mut max = 0;
            for j in (0..self.cols).rev() {
                if j == self.cols - 1 || self.heights[i][j] > max {
                    max = self.heights[i][j];
                    self.visibility.make_visible(i, j);
                }
            }
        }

        // from bottom
        for j in 0..self.cols {
            let mut max = 0;
            for i in (0..self.rows).rev() {
                if i == self.rows - 1 || self.heights[i][j] > max {
                    max = self.heights[i][j];
                    self.visibility.make_visible(i, j);
                }
            }
        }
    }

    fn max_scenic_score(&self) -> usize {
        let mut max = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                let score = self.check_tree(i, j);
                if score > max {
                    max = score;
                }
            }
        }
        max
    }

    fn check_tree(&self, row_idx: usize, col_idx: usize) -> usize {
        // Check how many trees are visible from this tree

        let tree_height = self.heights[row_idx][col_idx];

        // check left
        let mut left_score = 0;
        for k in (0..col_idx).rev() {
            left_score += 1;
            if self.heights[row_idx][k] >= tree_height {
                break;
            }
        }

        // check right
        let mut right_score = 0;
        for k in (col_idx + 1)..self.cols {
            right_score += 1;
            if self.heights[row_idx][k] >= tree_height {
                break;
            }
        }

        // check up
        let mut up_score = 0;
        for k in (0..row_idx).rev() {
            up_score += 1;
            if self.heights[k][col_idx] >= tree_height {
                break;
            }
        }

        // check down
        let mut down_score = 0;
        for k in (row_idx + 1)..self.rows {
            down_score += 1;
            if self.heights[k][col_idx] >= tree_height {
                break;
            }
        }
        println!(
            "checking tree with height: {} at pos {}, {} \nleft_score: {}, right_score: {}, up_score: {}, down_score: {}\n",
            tree_height, row_idx, col_idx, left_score, right_score, up_score, down_score
        );

        left_score * right_score * up_score * down_score
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::from_str(input).unwrap();
    grid.calculate_visibility();
    let score = grid.visibility.score();
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from_str(input).unwrap();
    let scenic_score = grid.max_scenic_score();
    Some(scenic_score)
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
        assert_eq!(part_two(&input), Some(8));
    }
}
