use std::collections::{HashSet, VecDeque};

const INPUT: &'static str = include_str!("../inputs/day15.txt");

pub(crate) fn run() {
    println!("day 15, output 1: {}", parse1(INPUT));
    println!("day 15, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut costs = vec![vec![u32::MAX; n_cols]; n_rows];
    costs[n_rows - 1][n_cols - 1] = grid[n_rows - 1][n_cols - 1];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(n_rows - 1, n_cols - 1)]);
    let mut in_queue: HashSet<(usize, usize)> = HashSet::new();

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if col != 0 {
            let current_cost = costs[row][col - 1];
            let new_cost = costs[row][col] + grid[row][col - 1];
            costs[row][col - 1] = current_cost.min(new_cost);
            let new_pt = ((row, col - 1));
            if !in_queue.contains(&new_pt) {
                queue.push_back(new_pt);
                in_queue.insert(new_pt);
            }
        }
        if row != 0 {
            let current_cost = costs[row - 1][col];
            let new_cost = costs[row][col] + grid[row - 1][col];
            costs[row - 1][col] = current_cost.min(new_cost);
            let new_pt = ((row - 1, col));
            if !in_queue.contains(&new_pt) {
                queue.push_back(new_pt);
                in_queue.insert(new_pt);
            }
        }
    }

    costs[0][0] - grid[0][0]
}

fn parse2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 40);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 0);
    }
}
