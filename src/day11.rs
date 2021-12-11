use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day11.txt");

pub(crate) fn run() {
    println!("day 11, output 1: {}", parse1(INPUT));
    println!("day 11, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut flash_count = 0;

    let mut og: OctoGrid = input.parse().unwrap();

    for _ in 1..=100 {
        let mut queue = VecDeque::new();

        // Increase all energy levels
        for row in 0..og.n_rows {
            for col in 0..og.n_cols {
                if og.increment(row, col) {
                    queue.push_back((row, col))
                }
            }
        }

        // Follow all downstream effects
        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            let min_row = 0.max(row as i32 - 1) as usize;
            let max_row = (row + 1).min(og.n_rows - 1);
            let min_col = 0.max(col as i32 - 1) as usize;
            let max_col = (col + 1).min(og.n_cols - 1);
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if !og.flashed[r][c] && og.increment(r, c) {
                        queue.push_back((r, c));
                    }
                }
            }
        }

        flash_count += og.count_flashes();
    }

    flash_count
}

fn parse2(input: &str) -> usize {
    let mut og: OctoGrid = input.parse().unwrap();

    for step in 1.. {
        let mut queue = VecDeque::new();

        // Increase all energy levels
        for row in 0..og.n_rows {
            for col in 0..og.n_cols {
                if og.increment(row, col) {
                    queue.push_back((row, col))
                }
            }
        }

        // Follow all downstream effects
        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            let min_row = 0.max(row as i32 - 1) as usize;
            let max_row = (row + 1).min(og.n_rows - 1);
            let min_col = 0.max(col as i32 - 1) as usize;
            let max_col = (col + 1).min(og.n_cols - 1);
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if !og.flashed[r][c] && og.increment(r, c) {
                        queue.push_back((r, c));
                    }
                }
            }
        }

        if og.count_flashes() == 100 {
            return step;
        }
    }

    unreachable!();
}

struct OctoGrid {
    grid: Vec<Vec<usize>>,
    n_rows: usize,
    n_cols: usize,
    flashed: Vec<Vec<bool>>,
}

impl OctoGrid {
    fn count_flashes(&mut self) -> usize {
        let n_flashed = self
            .flashed
            .iter()
            .map(|row| row.iter().filter(|em| **em).count())
            .sum();

        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|val| {
                if *val > 9 {
                    *val = 0;
                }
            })
        });
        self.flashed = vec![vec![false; self.n_cols]; self.n_rows];

        n_flashed
    }

    fn increment(&mut self, row: usize, col: usize) -> bool {
        self.grid[row][col] += 1;
        self.flashed[row][col] = self.grid[row][col] > 9;
        self.flashed[row][col]
    }
}

impl FromStr for OctoGrid {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        let flashed = vec![vec![false; n_cols]; n_rows];

        Ok(OctoGrid {
            grid,
            n_rows,
            n_cols,
            flashed,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 1656);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 195);
    }
}
