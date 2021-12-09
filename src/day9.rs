use std::collections::VecDeque;

const INPUT: &'static str = include_str!("../inputs/day9.txt");

pub(crate) fn run() {
    let res = parse1(INPUT);
    println!("day 9, output 1: {}", res);

    let res = parse2(INPUT);
    println!("day 9, output 2: {}", res);
}

fn parse1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut risk_sum = 0;

    for row in 0..n_rows {
        for col in 0..n_cols {
            let this = grid[row][col];
            let left = if col != 0 { grid[row][col - 1] } else { 10 };
            let right = if col != n_cols - 1 {
                grid[row][col + 1]
            } else {
                10
            };
            let up = if row != 0 { grid[row - 1][col] } else { 10 };
            let down = if row != n_rows - 1 {
                grid[row + 1][col]
            } else {
                10
            };

            if [left, right, up, down].iter().all(|x| *x > this) {
                risk_sum += this + 1;
            }
        }
    }

    risk_sum
}

fn parse2(input: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut basin_sizes: Vec<usize> = vec![];

    for row in 0..n_rows {
        for col in 0..n_cols {
            if grid[row][col] != 9 {
                let mut basin_size = 0;
                let mut queue = VecDeque::from(vec![(row, col)]);
                grid[row][col] = 9;

                while !queue.is_empty() {
                    let (r, c) = queue.pop_front().unwrap();
                    basin_size += 1;
                    if r > 0 && grid[r - 1][c] != 9 {
                        grid[r - 1][c] = 9;
                        queue.push_back((r - 1, c))
                    }
                    if r < n_rows - 1 && grid[r + 1][c] != 9 {
                        grid[r + 1][c] = 9;
                        queue.push_back((r + 1, c))
                    }
                    if c > 0 && grid[r][c - 1] != 9 {
                        grid[r][c - 1] = 9;
                        queue.push_back((r, c - 1))
                    }
                    if c < n_cols - 1 && grid[r][c + 1] != 9 {
                        grid[r][c + 1] = 9;
                        queue.push_back((r, c + 1))
                    }
                }

                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0..3].iter().product()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 15);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 1134);
    }
}
